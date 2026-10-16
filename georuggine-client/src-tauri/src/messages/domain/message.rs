use serde::{Deserialize, Serialize};

/// Modello entità di dominio per il messaggio
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Message {
    pub message_id: Option<i64>,
    pub sender_id: i64,
    pub receiver_id: Option<i64>,
    pub timestamp: String,
    pub contents: String,
}
