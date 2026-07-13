use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod database;
mod web;

use config::{Config, DeploymentMode};

#[derive(Clone)]
struct AppState {
    database: DatabaseConnection,
    deployment_mode: DeploymentMode,
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("mailforagents=info,tower_http=info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = Config::from_env().expect("invalid application configuration");
    let database = database::connect(&config.database_path)
        .await
        .expect("failed to initialize product SQLite database");
    let listener = TcpListener::bind(config.address)
        .await
        .expect("failed to bind product listener");

    info!(address = %config.address, mode = %config.deployment_mode.label(), "Mail for Agents listening");
    axum::serve(
        listener,
        web::router(AppState {
            database,
            deployment_mode: config.deployment_mode,
        }),
    )
    .await
    .expect("product server failed");
}
