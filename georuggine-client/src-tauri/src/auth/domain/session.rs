use serde::{Deserialize, Serialize};

/// Dati di sessione dell'utente autenticato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: i64,
    pub username: String,
    pub token: String,
}
