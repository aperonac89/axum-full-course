// LESSON 4

use axum::{Router, http::StatusCode, response::Redirect, routing::get};

// Exercise 3
pub fn admin_v0_routes() -> Router {
    Router::new().route("/", get(v0_hello))
}
// End of Excercise 3

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

// Exercise 3
pub async fn v0_hello() -> Redirect {
    Redirect::permanent("/v1")
}
// End of Exercise 4
pub async fn v1_hello() -> (StatusCode, &'static str) {
    (StatusCode::OK, "v1 hello")
}

pub async fn v2_hello() -> (StatusCode, &'static str) {
    (StatusCode::OK, "v2 hello")
}
