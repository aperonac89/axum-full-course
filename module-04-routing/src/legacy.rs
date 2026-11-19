// LESSON 6
use axum::{
    Router,
    routing::get,
    extract::Path,
};

pub fn legacy_routes() -> Router {
    Router::new()
        .route("/", get(legacy_hello))
        .route("/{*rest}", get(legacy_rest))
}

pub async fn legacy_hello() -> &'static str {
    "legacy service: mounted with nest_service, no prefix knowledge\n"
}

pub async fn legacy_rest(Path(rest): Path<String>) -> String {
    format!("legacy service rest mounted: '{rest}' (prefix /legacy was stripped)")
}