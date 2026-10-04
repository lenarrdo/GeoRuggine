use serde::{Deserialize, Serialize};

/// Request per registrazione e login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

/// Risposta di autenticazione restituita dal server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: Option<String>,
}

/// Coordinate geografiche inviate periodicamente al server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub user_id: i64,
    pub x_coord: i64,
    pub y_coord: i64,
    pub timestamp: String,
}

/// Messaggi di chat scambiati tra client e server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: Option<i64>,
    pub sender_id: i64,
    pub receiver_id: Option<i64>, // None = messaggio broadcast
    pub timestamp: String,
    pub contents: String,
}

/// Messaggi inviabili sul canale WebSocket verso il server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(PositionUpdate),
    Chat(ChatMessage),
}

/// Dati di sessione dell'utente autenticato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: i64,
    pub username: String,
    pub token: String,
}
