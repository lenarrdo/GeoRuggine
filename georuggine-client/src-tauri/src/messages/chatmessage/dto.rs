use serde::{Deserialize, Serialize};

/// Modello per i messaggi di testo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub contents: String,
    pub timestamp: String,
}

pub type ChatMessage = TextMessage;
