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

/// Modello per i messaggi di testo (scambiati tra client e server, o inviati dal server in broadcast)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub sender_id: u32,
    pub receiver_id: u32,
    pub contents: String,
    pub timestamp: String,
}

impl TextMessage {
    pub fn new(sender_id: u32, receiver_id: u32, contents: String) -> Self {
        TextMessage {
            sender_id,
            receiver_id,
            contents,
            timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        }
    }
}

/// Coordinate geografiche/emulate inviate dal client al server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub sender_id: u32,
    pub x_coord: f32,
    pub y_coord: f32,
    pub timestamp: String,
}
