pub mod services;
pub mod user_repository;

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{
    auth::services::auth_service::{AuthService, LoginError, RegisterError},
    shared::app_state::AppState,
    tui::AppEvent,
};

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

pub async fn handle_registration(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthResponse>)> {
    let username = body.username.as_deref().unwrap_or("");
    let password = body.password.as_deref().unwrap_or("");

    match AuthService::register(&state.db, username, password).await {
        Ok(user) => {
            state.tx.send(AppEvent::UserRegistered(user)).await.unwrap();
            Ok((
                StatusCode::CREATED,
                Json(AuthResponse {
                    message: "success".to_string(),
                    token: None,
                }),
            ))
        }
        Err(RegisterError::EmptyCredentials) => Err((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                message: "Both username and password are required.".to_string(),
                token: None,
            }),
        )),
        Err(RegisterError::UsernameExists) => Err((
            StatusCode::CONFLICT,
            Json(AuthResponse {
                message: "Username already exists.".to_string(),
                token: None,
            }),
        )),
        Err(RegisterError::HashError) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: "Internal Server Error.".to_string(),
                token: None,
            }),
        )),
        Err(RegisterError::DatabaseError(_)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: "Failed to register user.".to_string(),
                token: None,
            }),
        )),
    }
}

pub async fn handle_login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthResponse>)> {
    let username = body.username.as_deref().unwrap_or("");
    let password = body.password.as_deref().unwrap_or("");

    match AuthService::login(&state.db, username, password).await {
        Ok(token) => Ok((
            StatusCode::OK,
            Json(AuthResponse {
                message: "success".to_string(),
                token: Some(token),
            }),
        )),
        Err(LoginError::EmptyCredentials) => Err((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                message: "Both username and password are required.".to_string(),
                token: None,
            }),
        )),
        Err(LoginError::InvalidCredentials) => Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                message: "Invalid username or password".to_string(),
                token: None,
            }),
        )),
        Err(LoginError::InternalError(_)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: "Internal Server Error".to_string(),
                token: None,
            }),
        )),
    }
}
