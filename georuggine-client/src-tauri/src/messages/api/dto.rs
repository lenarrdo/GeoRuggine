use serde::{Deserialize, Serialize};

/// Messaggi di chat scambiati tra client e server (nessun ID in transito sulla rete)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub contents: String,
    pub timestamp: String,
}
