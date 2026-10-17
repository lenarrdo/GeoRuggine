pub mod message_repository;

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

/// Coordinate geografiche inviate dal client al server (il timestamp viene calcolato dal client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: i64,
    pub y_coord: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

