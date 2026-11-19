// LESSON 3

use axum::{Router, http::StatusCode, response::Json, routing::get};
use serde_json::{Value, json};

pub fn api_handlers() -> Router {
    Router::new()
        .route("/health", get(api_status))
        .fallback(api_falback)
}

pub async fn api_falback() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "unknown path",
            "hint": "the nested router owns fallbacks under its prefix"
        })),
    )
}

pub async fn api_status() -> (StatusCode, Json<Value>) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("datetime before 1970")
        .as_secs();

    (
        StatusCode::OK,
        Json(json!({"status": "up", "timesamp": now})),
    )
}
