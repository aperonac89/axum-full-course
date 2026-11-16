use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserView {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
}
#[derive(Serialize)]
pub struct HealthReport {
    pub status: String,
    pub epoch_seconds: u64,
}

#[derive(Serialize)]
pub struct TypeRow {
    pub returns: &'static str,
    pub status: u16,
    pub content_type: &'static str,
    pub note: &'static str,
}
