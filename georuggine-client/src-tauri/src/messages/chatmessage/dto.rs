use serde::{Deserialize, Serialize};

/// Model for text messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub contents: String,
    pub timestamp: String,
}

pub type ChatMessage = TextMessage;
