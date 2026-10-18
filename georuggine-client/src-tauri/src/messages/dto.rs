use crate::messages::chatmessage::dto::TextMessage;
use crate::messages::position::dto::PositionUpdate;
use serde::{Deserialize, Serialize};

/// Tipi di messaggi che il client può inviare al server via WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Text(TextMessage),
}
