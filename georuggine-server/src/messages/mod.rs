pub mod message_repository;

use serde::{Deserialize, Serialize};

/// Tipi di messaggi che il client può inviare al server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Text(TextMessage),
}

/// Modello per i messaggi di testo (scambiati tra client e server, o inviati dal server in broadcast)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub sender_id: i64,
    pub receiver_id: i64,
    pub timestamp: String,
    pub contents: String,
}

/// Coordinate geografiche/emulate inviate dal client al server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: i64,
    pub y_coord: i64,
    pub timestamp: String,
}
