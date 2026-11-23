use axum::{
    Router,
    extract::{Path, Query},
    response::Json,
    routing::post,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OrderPriority {
    priority: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OrderLine {
    sku: String,
    quantity: u32,
}

pub fn lesson4_routes() -> Router {
    Router::new().route("/orders/{order_id}", post(add_order_line))
}

async fn add_order_line(
    Path(order_id): Path<u32>,
    Query(priority): Query<OrderPriority>,
    Json(line): Json<OrderLine>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "order_id": order_id,
        "priority": priority.priority.unwrap_or_else(||"normal".to_string()),
        "sku": line.sku,
        "quantity": line.quantity,
        "note": "three extractor: Path then Query then Json(body)",
    }))
}
