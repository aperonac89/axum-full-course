mod admin_handler;
mod api_handlers;
mod handlers;
mod legacy;

use crate::admin_handler::{admin_merged_routes, admin_v1_routes, admin_v2_routes};
use crate::api_handlers::api_handlers;
use crate::handlers::{
    create_item, dashboard, delete_item, list_items, method_not_found_json, not_found_json,
    read_item, rename_item, wildcard_files,
};
use crate::legacy::legacy_routes;
use axum::{
    Router,
    routing::{get, post, put},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/items", get(list_items))
        .route(
            "/items/{item}",
            post(create_item).get(read_item).delete(delete_item),
        )
        .route("/items/{item_id}/{item_name}", put(rename_item))
        .route("/files/{*path}", get(wildcard_files))
        .nest("/api/v1", api_handlers())
        .merge(admin_merged_routes())
        .nest("/v1", admin_v1_routes())
        .nest("/v2", admin_v2_routes())
        .nest_service("/legacy", legacy_routes())
        .fallback(not_found_json)
        .method_not_allowed_fallback(method_not_found_json);

    let port = std::env::var("AXUM_PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3004);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap_or_else(|error| panic!("not possible to bind the port {port}: {error}"));

    println!("Axum server listening on http://localhost:{port}");
    axum::serve(listener, app)
        .await
        .expect("server stopped unexpectedly");
}
