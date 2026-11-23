use axum::{
    Router,
    routing::{get, post},
};

pub fn lesson9_routes() -> Router {
    Router::new()
        .route("/api-key", get(needs_api_key))
        .route("/api-key-optional", get(maybe_api_key))
        .route("/lines", post(plain_lines))
        .route("/client", get(client_summary))
}

async fn needs_api_key() {
    todo!()
}

async fn maybe_api_key() {
    todo!()
}

async fn plain_lines() {
    todo!()
}

async fn client_summary() {
    todo!()
}
