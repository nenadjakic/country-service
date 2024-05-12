use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn status() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");

    let response = json!({
        "version": version,
        "message": "Service is running...",
        "status": "OK"
    });
    (StatusCode::OK, Json(response))
}