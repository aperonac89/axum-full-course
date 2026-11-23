use axum::{Router, extract::Form, response::Json, routing::post};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Feedback {
    name: String,
    rating: u8,
    comment: Option<String>,
}

pub fn lesson5_routes() -> Router {
    Router::new().route("/feedback", post(feedback))
}

async fn feedback(form: Form<Feedback>) -> Json<serde_json::Value> {
    Json(json!({
        "received_from": form.name,
        "rating": form.rating,
        "comment": form.comment,
        "note": "Form extractor: application/x-www-form-urlencoded"
    }))
}
