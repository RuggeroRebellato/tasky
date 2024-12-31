use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(id: u32, description: String, tags: Vec<String>) -> Self {
        Task {
            id,
            description,
            done: false,
            tags,
        }
    }
}
