use crate::messages::api::dto::ChatMessage;
use crate::tracking::api::dto::PositionUpdate;
use serde::{Deserialize, Serialize};

/// Messaggi inviabili sul canale WebSocket verso il server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Chat(ChatMessage),
}
