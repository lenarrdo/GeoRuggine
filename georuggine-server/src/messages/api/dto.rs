use serde::{Deserialize, Serialize};

// Re-export PositionUpdate per retrocompatibilità e protocollo websocket
pub use crate::tracking::PositionUpdate;

/// Modello per i messaggi di testo (scambiati tra client e server, o inviati dal server in broadcast)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: Option<i64>,
    pub sender_id: i64,
    pub receiver_id: Option<i64>, // None indica broadcast dal server all'intera flotta
    pub timestamp: String,
    pub contents: String,
}

/// Tipi di messaggi che il client può inviare al server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Chat(ChatMessage),
}
