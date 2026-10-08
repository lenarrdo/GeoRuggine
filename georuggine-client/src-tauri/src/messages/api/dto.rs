use serde::{Deserialize, Serialize};

/// Messaggi di chat scambiati tra client e server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: Option<i64>,
    pub sender_id: i64,
    pub receiver_id: Option<i64>, // None = messaggio broadcast
    pub timestamp: String,
    pub contents: String,
}
