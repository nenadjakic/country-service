use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::config::app_state::AppState;

pub async fn find_by_id(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    app_state
        .lock()
        .await
        .country_service
        .find_by_id(id)
        .map_or_else(
            || {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "message": "NOT_FOUND"})),
                )
            },
            |country| (StatusCode::OK, Json(json!(country))),
        )
}

pub async fn find_by_alpha2_code(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    app_state
        .lock()
        .await
        .country_service
        .find_by_alpha2_code(code)
        .map_or_else(
            || {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "message": "NOT_FOUND"})),
                )
            },
            |country| (StatusCode::OK, Json(json!(country))),
        )
}

pub async fn find_by_name(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let countries = app_state.lock().await.country_service.find_by_name(name);

    (StatusCode::OK, Json(json!(countries)))
}

pub async fn find_all(State(app_state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let countries = app_state.lock().await.country_service.find_all();

    (StatusCode::OK, Json(json!(countries)))
}
