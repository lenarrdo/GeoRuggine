pub mod position;
pub mod text;

use serde::{Deserialize, Serialize};

use crate::messages::{position::Coordinates, text::TextMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(Coordinates),
    Text(TextMessage),
}
