use axum::{Router, http::StatusCode, response::Json, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct UserInput {
    username: String,
    age_years: u16,
}

#[derive(Debug, Serialize)]
struct UserCreated {
    created: bool,
    username: String,
    age_years: u16,
}

pub fn lesson3_routes() -> Router {
    Router::new().route("/users", post(create_user))
}

async fn create_user(Json(user_input): Json<UserInput>) -> (StatusCode, Json<UserCreated>) {
    (
        StatusCode::CREATED,
        Json(UserCreated {
            created: true,
            username: user_input.username,
            age_years: user_input.age_years,
        }),
    )
}
