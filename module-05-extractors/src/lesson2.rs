use axum::{Router, extract::Query, response::Json, routing::get};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Debug, Deserialize)]
struct SearchParams {
    q: String,
    page: Option<u32>,
    sort: Option<String>,
}

pub fn lesson2_routes() -> Router {
    Router::new().route("/search", get(search))
}

async fn search(Query(params): Query<SearchParams>) -> Json<Value> {
    Json(json!({
        "q": params.q,
        "page": params.page.unwrap_or(1),
        "sort": params.sort.unwrap_or_else(|| "relevance".to_string()),
        "note": "page/sort were optional, q was required"
    }))
}
