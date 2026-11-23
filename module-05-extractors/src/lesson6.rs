use axum::{
    Router,
    extract::{Path, rejection::JsonRejection},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Greeting {
    message: String,
}

pub fn lesson6_routes() -> Router {
    Router::new()
        .route("/theme", get(theme_prefs))
        .route("/theme/{name}", get(theme_prefs))
        .route("/optional-json", get(optional_json))
        .route("/strict-json", post(strict_json))
}

async fn theme_prefs(path: Option<Path<String>>) -> Json<Value> {
    match path {
        Some(Path(name)) => Json(json!({
            "chosen_theme": name,
            "note": "the path segment existed - Some"
        })),
        None => Json(json!({
            "chosen_theme": "default",
            "note": "the path segment does not exist - None"
        })),
    }
}

async fn optional_json(body: Option<Json<Greeting>>) -> String {
    match body {
        Some(greeting) => format!("you said: {}", greeting.message),
        None => "no body sent - Option<Json> was none, so no rejection\n".to_string(),
    }
}

async fn strict_json(body: Result<Json<Greeting>, JsonRejection>) -> (StatusCode, String) {
    match body {
        Ok(greeting) => (
            StatusCode::OK,
            format!("strict-json ok: {}\n", greeting.message),
        ),
        Err(rejection) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("rejection made into a value: {rejection}\n"),
        ),
    }
}
