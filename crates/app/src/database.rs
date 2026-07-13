use std::{path::Path, time::Duration};

use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
    sqlx::sqlite::{SqliteJournalMode, SqliteSynchronous},
};

pub async fn connect(path: &Path) -> Result<DatabaseConnection, sea_orm::DbErr> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| sea_orm::DbErr::Custom(error.to_string()))?;
    }

    let url = format!("sqlite://{}?mode=rwc", path.display());
    let mut options = ConnectOptions::new(url);
    options
        .max_connections(8)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(|options| {
            options
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(Duration::from_secs(5))
        });

    let database = Database::connect(options).await?;
    migrate(&database).await?;
    verify(&database).await?;
    tracing::info!(path = %path.display(), "SQLite product database ready");
    Ok(database)
}

async fn migrate(database: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    database
        .execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY,
                slug TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'suspended')),
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                display_name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS memberships (
                workspace_id TEXT NOT NULL,
                user_id TEXT NOT NULL,
                role TEXT NOT NULL CHECK (
                    role IN ('owner', 'administrator', 'marketer', 'developer',
                             'analyst', 'reviewer', 'read_only')
                ),
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (workspace_id, user_id),
                FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS memberships_user_workspace
                ON memberships (user_id, workspace_id);

            CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                workspace_id TEXT NOT NULL,
                actor_type TEXT NOT NULL,
                actor_id TEXT,
                action TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE RESTRICT
            );

            CREATE INDEX IF NOT EXISTS audit_logs_workspace_created
                ON audit_logs (workspace_id, created_at DESC);

            INSERT OR IGNORE INTO schema_migrations (version) VALUES (1);
            "#,
        )
        .await?;
    Ok(())
}

async fn verify(database: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    let foreign_keys = database
        .query_one(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA foreign_keys",
        ))
        .await?
        .and_then(|row| row.try_get_by_index::<i32>(0).ok())
        .unwrap_or_default();

    if foreign_keys != 1 {
        return Err(sea_orm::DbErr::Custom(
            "SQLite foreign key enforcement is disabled".into(),
        ));
    }

    let result = database
        .query_one(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA quick_check",
        ))
        .await?
        .and_then(|row| row.try_get_by_index::<String>(0).ok())
        .unwrap_or_default();
    if result != "ok" {
        return Err(sea_orm::DbErr::Custom(format!(
            "SQLite quick_check failed: {result}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn enables_pragmas_and_applies_workspace_schema() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let database = connect(&directory.path().join("test.sqlite"))
            .await
            .expect("SQLite connection");

        let journal_mode = database
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "PRAGMA journal_mode",
            ))
            .await
            .expect("journal mode query")
            .and_then(|row| row.try_get_by_index::<String>(0).ok())
            .expect("journal mode value");
        assert_eq!(journal_mode.to_ascii_lowercase(), "wal");

        let workspace_table = database
            .query_one(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'workspaces'",
            ))
            .await
            .expect("schema query");
        assert!(workspace_table.is_some());
    }

    #[tokio::test]
    async fn membership_cannot_escape_workspace_and_user_foreign_keys() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let database = connect(&directory.path().join("test.sqlite"))
            .await
            .expect("SQLite connection");

        let result = database
            .execute_unprepared(
                "INSERT INTO memberships (workspace_id, user_id, role) \
                 VALUES ('ws_missing', 'usr_missing', 'owner')",
            )
            .await;
        assert!(result.is_err());
    }
}
