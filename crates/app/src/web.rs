use askama::Template;
use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::AppState;

const CONSOLE_CSS: &str = include_str!("../static/console.css");

#[derive(Template)]
#[template(path = "console.html")]
struct ConsoleTemplate {
    deployment_label: &'static str,
}

#[derive(Serialize)]
struct Capabilities {
    product: &'static str,
    version: &'static str,
    multi_workspace: bool,
    persistence: &'static str,
    provisioning: &'static str,
    authentication: &'static str,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(console))
        .route("/healthz", get(|| async { "ok" }))
        .route("/readyz", get(readiness))
        .route("/api/v1/capabilities", get(capabilities))
        .route("/assets/console.css", get(console_css))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn console(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    ConsoleTemplate {
        deployment_label: state.deployment_mode.label(),
    }
    .render()
    .map(Html)
    .map_err(|error| {
        tracing::error!(%error, "failed to render product console");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn readiness(State(state): State<AppState>) -> Result<&'static str, StatusCode> {
    state
        .database
        .query_one(Statement::from_string(DbBackend::Sqlite, "SELECT 1"))
        .await
        .map(|_| "ready")
        .map_err(|error| {
            tracing::error!(%error, "product SQLite readiness check failed");
            StatusCode::SERVICE_UNAVAILABLE
        })
}

async fn capabilities() -> Json<Capabilities> {
    Json(Capabilities {
        product: "mailforagents",
        version: env!("CARGO_PKG_VERSION"),
        multi_workspace: true,
        persistence: "sqlite",
        provisioning: "cloud-controller-or-self-hosted-admin",
        authentication: "not-yet-enabled",
    })
}

async fn console_css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        CONSOLE_CSS,
    )
}

async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Html(
            "<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><title>Not found · Mail for Agents</title><body><main><h1>Not found</h1><p><a href=\"/\">Return to the console</a></p></main></body></html>"
                .to_owned(),
        ),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::{config::DeploymentMode, database};

    async fn test_app() -> (Router, tempfile::TempDir) {
        let directory = tempfile::tempdir().expect("temporary directory");
        let path = directory.path().join("app.sqlite");
        let database = database::connect(&path).await.expect("SQLite connection");
        (
            router(AppState {
                database,
                deployment_mode: DeploymentMode::SelfHosted,
            }),
            directory,
        )
    }

    #[tokio::test]
    async fn console_is_server_rendered_and_honest_about_scaffold_state() {
        let (app, _directory) = test_app().await;
        let response = app
            .oneshot(Request::get("/").body(Body::empty()).expect("request"))
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::OK);
        let body = response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let html = String::from_utf8(body.to_vec()).expect("UTF-8 body");
        assert!(html.contains("Provisioning required"));
        assert!(html.contains("Self-hosted"));
        assert!(html.contains("AGPL-3.0-or-later"));
        assert!(html.contains("https://github.com/mailforagents/mailforagents"));
        assert!(!html.contains("Create workflow</button>"));
    }

    #[tokio::test]
    async fn readiness_checks_the_disposable_database() {
        let (app, _directory) = test_app().await;
        let response = app
            .oneshot(
                Request::get("/readyz")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::OK);
    }
}
