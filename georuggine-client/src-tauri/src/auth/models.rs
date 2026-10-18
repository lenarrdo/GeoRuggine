pub mod dto {
    use serde::{Deserialize, Serialize};

    /// Richiesta inviata al server HTTP.
    #[derive(Debug, Clone, Serialize)]
    pub struct AuthRequest {
        pub username: String,
        pub password: String,
    }

    /// Risposta ricevuta dal server HTTP.
    #[derive(Debug, Clone, Deserialize)]
    pub struct AuthResponse {
        pub message: String,
        pub token: Option<String>,
    }

    /// Risultato del login esposto al frontend.
    #[derive(Debug, Clone, Serialize)]
    pub struct AuthResult {
        pub success: bool,
        pub message: String,
    }
}

pub mod session {
    /// Dati di sessione interni al backend Rust.
    #[derive(Debug, Clone)]
    pub struct UserSession {
        pub username: String,
        pub token: String,
    }
}

pub use dto::{AuthRequest, AuthResponse, AuthResult};
pub use session::UserSession;
