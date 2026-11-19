// LESSON 4

use axum::{Router, http::StatusCode, routing::get};

pub fn admin_v1_routes() -> Router {
    Router::new().route("/", get(v1_hello))
}

pub fn admin_v2_routes() -> Router {
    Router::new().route("/", get(v2_hello))
}

pub fn admin_merged_routes() -> Router {
    Router::new()
        .route("/admin/keys", get(admin_keys))
        .reset_fallback()
}

pub async fn admin_keys() -> (StatusCode, &'static str) {
    (StatusCode::OK, "API KEY not found in context")
}

pub async fn v1_hello() -> (StatusCode, &'static str) {
    (StatusCode::OK, "v1 hello")
}

pub async fn v2_hello() -> (StatusCode, &'static str) {
    (StatusCode::OK, "v2 hello")
}
