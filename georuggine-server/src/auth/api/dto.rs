use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: Option<String>,
}
