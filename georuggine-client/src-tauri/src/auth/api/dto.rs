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
