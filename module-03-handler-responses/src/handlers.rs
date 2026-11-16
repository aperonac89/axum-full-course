use crate::models::{HealthReport, TypeRow, UserView};
use axum::http::{StatusCode, header};
use axum::response::{Html, IntoResponse, Json, NoContent, Redirect, Response};

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
pub async fn health() -> Json<HealthReport> {
    Json(HealthReport {
        status: "Running".to_string(),
        epoch_seconds: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before 1970")
            .as_secs(),
    })
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
pub async fn json_created() -> (StatusCode, Json<UserView>) {
    (
        StatusCode::CREATED,
        Json(UserView {
            id: 2,
            username: "new-user".to_string(),
            is_admin: false,
        }),
    )
}

pub async fn json_user() -> Json<UserView> {
    Json(UserView {
        id: 1234,
        username: "Alex".to_string(),
        is_admin: true,
    })
}
pub async fn maybe_response(request: axum::extract::Request) -> Response {
    let query = request.uri().query().unwrap_or("");
    if query.contains("mode=json") {
        Json(serde_json::json!({"mode": "json"})).into_response()
    } else if query.contains("mode=html") {
        Html("<p>mode = <b>html</b></p>").into_response()
    } else {
        "mode = text\n".into_response()
    }
}
pub async fn no_content() -> NoContent {
    NoContent
}
pub async fn plain_bytes() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        &b"raw bytes, labelled as text\n"[..],
    )
}
pub async fn redirect_permanent() -> Redirect {
    Redirect::permanent("/")
}
pub async fn redirect_see_other() -> Redirect {
    Redirect::to("/page")
}
pub async fn redirect_temporary() -> Redirect {
    Redirect::temporary("/page")
}
pub async fn response_types() -> Json<Vec<TypeRow>> {
    Json(vec![
        TypeRow {
            returns: "&'static str / String",
            status: 200,
            content_type: "text/plain; charset=utf-8",
            note: "dynamic text -> String",
        },
        TypeRow {
            returns: "Html<T>",
            status: 200,
            content_type: "text/html; charset=utf-8",
            note: "T = &str or String",
        },
        TypeRow {
            returns: "Json<T: Serialize>",
            status: 200,
            content_type: "application/json",
            note: "serialisation failure -> 500",
        },
        TypeRow {
            returns: "StatusCode",
            status: 0,
            content_type: "(none)",
            note: "status only, empty body",
        },
        TypeRow {
            returns: "(StatusCode, T)",
            status: 0,
            content_type: "from T",
            note: "first element overrides the status",
        },
        TypeRow {
            returns: "NoContent",
            status: 204,
            content_type: "(none)",
            note: "the polite 'done, nothing to say'",
        },
        TypeRow {
            returns: "Redirect::to",
            status: 303,
            content_type: "(none)",
            note: "after a POST, go GET elsewhere",
        },
        TypeRow {
            returns: "Redirect::temporary",
            status: 307,
            content_type: "(none)",
            note: "same method, short-lived move",
        },
        TypeRow {
            returns: "Redirect::permanent",
            status: 308,
            content_type: "(none)",
            note: "same method, forever",
        },
        TypeRow {
            returns: "Vec<u8> / Bytes / &'static [u8]",
            status: 200,
            content_type: "application/octet-stream",
            note: "set your own type with a header tuple",
        },
        TypeRow {
            returns: "Result<T, E>",
            status: 0,
            content_type: "from the arm",
            note: "both arms must be IntoResponse (Module 08)",
        },
        TypeRow {
            returns: "()",
            status: 200,
            content_type: "(none)",
            note: "empty 200",
        },
    ])
}
pub async fn teapot_with_body() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "I am a teapot")
}
