use axum::{Router, extract::Path, routing::get};

pub fn lesson1_routes() -> Router {
    Router::new()
        .route("/products/{id}", get(product_by_id))
        .route("/users/{user_id}/posts/{post_id}", get(nested_params))
}

pub async fn product_by_id(Path(id): Path<u32>) -> String {
    format!("Path<u32> extracted: product id -> {id}")
}

pub async fn nested_params(Path((user_id, post_id)): Path<(u32, u32)>) -> String {
    format!("Path<(u32, u32)> extracted:\n\tuser_id -> {user_id}\n\tpost_id: {post_id}")
}
