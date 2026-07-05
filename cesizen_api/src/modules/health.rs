use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

pub async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<serde_json::Value>) {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "status": "ok", "database": "ok" })),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "status": "degraded", "database": "unreachable" })),
        ),
    }
}