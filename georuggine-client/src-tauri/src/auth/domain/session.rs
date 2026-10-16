use serde::{Deserialize, Serialize};

/// Dati di sessione dell'utente autenticato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub username: String,
    pub token: String,
}
