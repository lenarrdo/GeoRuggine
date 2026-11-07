pub mod repository;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub contents: String,
    pub timestamp: DateTime<Utc>,
}

impl TextMessage {
    pub fn new(contents: String) -> Self {
        TextMessage {
            contents,
            timestamp: Utc::now(),
        }
    }
}
