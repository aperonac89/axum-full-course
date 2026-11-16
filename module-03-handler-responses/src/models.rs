use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserView {
    pub id: i64,
    pub username: String,
    pub is_admin: bool,
}