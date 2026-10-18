pub mod message_repository;

use chrono::Local;
use serde::{Deserialize, Serialize};

/// Tipi di messaggi che il client può inviare al server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Text(TextMessage),
}

/// Modello per i messaggi di testo (client e server inviano e ricevono entrambi contents e timestamp generato in quel momento)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub contents: String,
    pub timestamp: String,
}

impl TextMessage {
    pub fn new(contents: String) -> Self {
        TextMessage {
            contents,
            timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

/// Coordinate geografiche inviate dal client al server (il timestamp viene calcolato dal client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: f64,
    pub y_coord: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
