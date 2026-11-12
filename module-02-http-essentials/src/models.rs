use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct RepoLookup {
    pub name: String,
    pub stars: u16,
    pub topics: Vec<String>,
}

pub trait Describe {
    fn describe(&self) -> String;
}

pub struct Student {
    pub name: &'static str,
}

pub struct Course {
    pub title: &'static str,
    pub modules: u8,
}

impl Describe for Student {
    fn describe(&self) -> String {
        format!("student {}", self.name)
    }
}

impl Describe for Course {
    fn describe(&self) -> String {
        format!("course {} with {} modules", self.title, self.modules)
    }
}

pub fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    items.iter().map(Describe::describe).collect()
}

pub fn shout(item: impl Display) -> String {
    item.to_string().to_uppercase()
}

#[derive(Serialize)]
pub struct RestRow {
    pub method: &'static str,
    pub path: &'static str,
    pub meaning: &'static str,
    pub success: u16,
    pub idempotent: bool,
    pub safe: bool,
}
