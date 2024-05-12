use axum::routing::get;
use axum::Router;
use dotenv::dotenv;
use envconfig::Envconfig;
use log::info;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::config::app_settings::AppSettings;
use crate::config::app_state::AppState;
use crate::controller::country_controller::{
    find_all, find_by_alpha2_code, find_by_id, find_by_name,
};
use crate::controller::health_controller::status;
use crate::util::{establish_connection, run_pending_migrations};

mod config;
mod controller;
mod entity;
mod repository;
mod schema;
mod service;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let app_settings = AppSettings::init_from_env().unwrap();

    info!("🚀 Starting...");

    let bind_address = String::from("0.0.0.0") + ":" + &app_settings.server_port().to_string();

    let db_pool = establish_connection(app_settings.database_url().to_string());
    info!("Database connection established.");

    run_pending_migrations(db_pool.get().unwrap());
    info!("Database migrations run.");

    let app_state = Arc::new(Mutex::new(AppState::new(db_pool)));

    let app = Router::new()
        .route("/health/status", get(status))
        .route("/countries/:country_id", get(find_by_id))
        .route("/countries", get(find_all))
        .route("/countries/by_code/:code", get(find_by_alpha2_code))
        .route("/countries/by_name/:name", get(find_by_name))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    info!("Listening on port {}.", &app_settings.server_port());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("Server stopped.");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Signal received, starting graceful shutdown");
}
