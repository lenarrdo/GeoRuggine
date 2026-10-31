pub mod repository;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub contents: String,
    pub timestamp: String,
}

impl TextMessage {
    pub fn new(contents: String) -> Self {
        TextMessage {
            contents,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
