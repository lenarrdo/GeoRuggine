use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: u32,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl AuthRequest {
    /// Validates presence and non-emptiness of credentials
    pub fn validate(&self) -> Result<(&str, &str), &'static str> {
        let username = self.username.as_deref().unwrap_or("").trim();
        let password = self.password.as_deref().unwrap_or("").trim();
        if username.is_empty() || password.is_empty() {
            Err("Both username and password are required.")
        } else {
            Ok((username, password))
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl AuthResponse {
    pub fn success() -> Self {
        Self {
            message: "success".to_string(),
            token: None,
        }
    }
    pub fn with_token(token: String) -> Self {
        Self {
            message: "success".to_string(),
            token: Some(token),
        }
    }
}

#[derive(Debug)]
pub enum RegisterError {
    EmptyCredentials(&'static str),
    UsernameAlreadyExists,
    Internal(String),
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RegisterError::EmptyCredentials(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            RegisterError::UsernameAlreadyExists => {
                (StatusCode::CONFLICT, "Username already exists.".to_string())
            }
            RegisterError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {err}"),
            ),
        };
        (
            status,
            Json(AuthResponse {
                message,
                token: None,
            }),
        )
            .into_response()
    }
}

#[derive(Debug)]
pub enum LoginError {
    EmptyCredentials(&'static str),
    InvalidCredentials,
    Internal(String),
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            LoginError::EmptyCredentials(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            LoginError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            ),
            LoginError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {err}"),
            ),
        };
        (
            status,
            Json(AuthResponse {
                message,
                token: None,
            }),
        )
            .into_response()
    }
}
