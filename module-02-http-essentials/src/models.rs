use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RepoLookup {
    pub name: String,
    pub stars: u16,
    pub topics: Vec<String>,
}
