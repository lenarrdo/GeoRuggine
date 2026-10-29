use crate::messages::chatmessage::dto::TextMessage;
use crate::messages::position::dto::PositionUpdate;
use serde::{Deserialize, Serialize};

/// Message types the client can send to the server over WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Text(TextMessage),
}
