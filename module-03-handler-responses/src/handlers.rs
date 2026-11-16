use crate::models::UserView;
use axum::http::StatusCode;
use axum::response::{Html, Json, NoContent, Redirect};

pub async fn dashboard() -> Html<&'static str> {
    Html(
        "<h1>Module 03 — handlers and responses</h1>\
         <ul>\
         <li>GET /greeting — String, built at request time</li>\
         <li>GET /page — Html static</li>\
         <li>GET /user — Json</li>\
         <li>GET /created — (201 Created, body) in a tuple</li>\
         <li>GET /teapot — (418, body) in a tuple</li>\
         <li>GET /no-content — 204 NoContent, empty body</li>\
         <li>GET /old-page — Redirect::to (303 See Other)</li>\
         <li>GET /moved — Redirect::permanent (308)</li>\
         <li>GET /health — the health check</li>\
         <li>GET /maybe?mode=json|html|text — impl IntoResponse with different branches</li>\
         <li>GET /types — every built-in return type in one table</li>\
         <li>GET /temporary — Redirect::temporary (307)</li>\
         <li>GET /json-created — (201, Json) tuple</li>\
         <li>GET /plain-bytes — Vec&lt;u8&gt; / [u8] bodies with a content type</li>\
         </ul>",
    )
}
pub async fn created_with_body() -> (StatusCode, &'static str) {
    (
        StatusCode::CREATED,
        "Created(201). The body is the second part of the tuple",
    )
}

pub async fn greetings() -> String {
    let epoch = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock before 1970")
        .as_secs();

    format!("Greeting handler with epoch time: {epoch}")
}
pub async fn health() {
    todo!()
}
pub async fn html_page() -> Html<&'static str> {
    Html(
        "<!doctype html>\
         <html lang=\"en\">\
         <head><meta charset=\"utf-8\"><title>Module 03</title>\
         <style>body{font-family:system-ui;max-width:640px;margin:2rem auto;color:#333}</style></head>\
         <body>\
         <h1>Served by Axum, rendered by your browser</h1>\
         <p>This whole page is a <code>&'static str</code> wrapped in <code>Html</code>.</p>\
         <p>Clicking below leaves this page, hits <code>/old-page</code>, and the server answers <code>303 See Other</code> with <code>location: /page</code> — your browser quietly goes there.</p>\
         <p><a href=\"old-page\">follow the redirect</a></p>\
         </body></html>",
    )
}
pub async fn json_created() {
    todo!()
}
pub async fn json_user() -> Json<UserView> {
    Json(UserView {
        id: 1234,
        username: "Alex".to_string(),
        is_admin: true,
    })
}
pub async fn maybe_response() {
    todo!()
}
pub async fn no_content() -> NoContent {
    NoContent
}
pub async fn plain_bytes() {
    todo!()
}
pub async fn redirect_permanent() -> Redirect {
    Redirect::permanent("/")
}
pub async fn redirect_see_other() -> Redirect {
    Redirect::to("/page")
}
pub async fn redirect_temporary() {
    todo!()
}
pub async fn response_types() {
    todo!()
}
pub async fn teapot_with_body() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "I am a teapot")
}
