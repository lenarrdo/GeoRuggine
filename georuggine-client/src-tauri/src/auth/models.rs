pub mod dto {
    use serde::{Deserialize, Serialize};

    /// Request sent to the HTTP server.
    #[derive(Debug, Clone, Serialize)]
    pub struct AuthRequest {
        pub username: String,
        pub password: String,
    }

    /// Response received from the HTTP server.
    #[derive(Debug, Clone, Deserialize)]
    pub struct AuthResponse {
        pub message: String,
        pub token: Option<String>,
    }

    /// Login outcome exposed to the frontend.
    #[derive(Debug, Clone, Serialize)]
    pub struct AuthResult {
        pub success: bool,
        pub message: String,
    }
}

pub mod session {
    /// Session data internal to the Rust backend.
    #[derive(Debug, Clone)]
    pub struct UserSession {
        pub username: String,
        pub token: String,
    }
}

pub use dto::{AuthRequest, AuthResponse, AuthResult};
pub use session::UserSession;
