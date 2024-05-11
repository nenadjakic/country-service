use std::env;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use log::info;
use serde_json::json;
use tokio::signal::{self, unix::signal};

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    info!("🚀 Server starting...");

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("80".to_string());

    let bind_address = app_host + ":" + &app_port;

    let app = Router::new()
        .route("/api/health/status", get(status));

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("Server stopped.");
}

async fn status() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");

    let response = json!({
        "version": version,
        "message": "Service is running...",
        "status": "OK"
    });
    (StatusCode::OK, Json(response))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
