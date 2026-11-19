// Leson 1 - 2
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Json},
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Mutex;

pub async fn dashboard() -> Html<&'static str> {
    Html(
        "<h1>Module 04 — Routing</h1>\
         <ul>\
         <li>CRUD: GET /items &middot; POST /items/{name} &middot; GET|DELETE /items/{id} &middot; PUT /items/{id}/{name}</li>\
         <li>Wildcard: GET /files/{*path} — try /files/assets/css/main.css</li>\
         <li>Nest: GET /api/status — and GET /api/anything-else hits the nested fallback</li>\
         <li>Merge: GET /admin/keys (merge adds routes, no prefix; reset_fallback demo)</li>\
         <li>Versioning: GET /v1/hello vs GET /v2/hello</li>\
         <li>Service mount: GET /legacy/anything</li>\
         <li>Fallbacks: any unknown path — and PATCH /items/1 for the 405 one</li>\
         <li>any(): ANY /echo-method &middot; on(): GET|POST /read-or-write &middot; HEAD: /head-demo &middot; typed: /typed-items/{id}</li>\
         <li>Trailing slash: /docs and /docs/ &middot; services: /service, /svc/known, /svc/anything</li>\
         </ul>",
    )
}

fn item_store() -> &'static Mutex<HashMap<u32, String>> {
    static ITEMS: std::sync::OnceLock<Mutex<HashMap<u32, String>>> = std::sync::OnceLock::new();
    ITEMS.get_or_init(|| {
        Mutex::new(HashMap::from([
            (1_u32, "ring".to_string()),
            (2_u32, "amulet".to_string()),
        ]))
    })
}

pub async fn list_items() -> String {
    let pairs: Vec<String> = item_store()
        .lock()
        .expect("store lock is fine")
        .iter()
        .map(|(id, name)| format!("{id}: {name}"))
        .collect();
    format!("items now: {}\n", pairs.join(", "))
}

pub async fn create_item(Path(name): Path<String>) -> (StatusCode, String) {
    let mut store = item_store().lock().expect("store lock is fine");
    let next_id = store.keys().max().copied().unwrap_or(0) + 1;
    let inserted = store.insert(next_id, name.clone());
    drop(store);
    match inserted {
        None => (
            StatusCode::CREATED,
            format!("created item {next_id}: '{name}'\n"),
        ),
        Some(old) => (
            StatusCode::CONFLICT,
            format!("id {next_id} already held '{old}'\n"),
        ),
    }
}

pub async fn read_item(Path(item_id): Path<String>) -> (StatusCode, String) {
    match item_id.parse::<u32>() {
        Ok(id) => match item_store().lock().expect("lock is fine").get(&id) {
            Some(item) => (StatusCode::OK, format!("item with code: {id} is: {item}")),
            None => (
                StatusCode::NOT_FOUND,
                format!("item with code: {id} not found"),
            ),
        },
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("element introduced is not an item id"),
        ),
    }
}

pub async fn rename_item(
    Path((item_id, item_name)): Path<(String, String)>,
) -> (StatusCode, String) {
    match item_id.parse::<u32>() {
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("element introduced is not an item id"),
        ),
        Ok(id) => {
            let old = item_store()
                .lock()
                .expect("lock is fine")
                .insert(id, item_name.clone());
            match old {
                Some(_) => (StatusCode::OK, format!("item {id} renamed to {item_name}")),
                None => (StatusCode::NOT_FOUND, format!("no item {id} to be renamed")),
            }
        }
    }
}

pub async fn delete_item(Path(item_id): Path<String>) -> (StatusCode, String) {
    match item_id.parse::<u32>() {
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("Element introduced is not an item id"),
        ),
        Ok(id) => match item_store().lock().expect("lock is fine").remove(&id) {
            None => (StatusCode::NOT_FOUND, format!("no item with id {id}")),
            Some(name) => (
                StatusCode::NO_CONTENT,
                format!("item {id} with name {name} has been removed"),
            ),
        },
    }
}

// ============================================================================
// LESSON 2: The wildcard route ({*path} matches whole remainder)
// ============================================================================

/// GET /files/{*path} — the wildcard row.
///
/// What it does: matches `/files/a/b/c.css` and hands the handler the whole
/// remainder. Try `curl -s http://127.0.0.1:3004/files/assets/css/main.css`.
/// Why it matters: `{*path}` is the 0.8 wildcard syntax (old `*path` panics at
/// startup now). Params and wildcards — plus this same `Path` extractor with
/// typed targets and proper rejections — are Module 05's core lesson; this
/// row just proves routing's vocabulary first. Module 13 turns exactly this
/// row into real `ServeDir` file serving via `nest_service`.
pub async fn wildcard_files(Path(filepath): Path<String>) -> String {
    format!("wildcard matched remainder: {filepath}")
}

// LESSON 5: NOT FOUND
pub async fn not_found_json() -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "Path provided wrong",
            "hint": "Please check if the path is correct"
        })),
    )
}

pub async fn method_not_found_json() -> (StatusCode, Json<Value>) {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(json!({
            "error": "Method not allowed",
            "hint": "Please check api documentation to check which methos are allowed for this endpoint"
        })),
    )
}
