use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Message {
    pub message_id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub timestamp: String,
    pub contents: Option<String>,
}
