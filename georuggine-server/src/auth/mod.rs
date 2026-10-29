pub mod auth_service;
pub mod dto;
pub mod user_repository;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::{
    auth::{
        auth_service::AuthService,
        dto::{AuthRequest, AuthResponse, LoginError, RegisterError},
    },
    shared::app_state::AppState,
    tui::{AppEvent, user_list::User},
};

pub async fn handle_registration(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, RegisterError> {
    let (username, password) = body.validate().map_err(RegisterError::EmptyCredentials)?;

    let auth_service = AuthService::new(state.db.clone(), state.config.jwt_secret.clone());
    let user_id = auth_service.register(username, password).await?;

    let _ = state
        .tx
        .send(AppEvent::UserRegistered(User::new(user_id, username)))
        .await;

    Ok((StatusCode::CREATED, Json(AuthResponse::success())))
}

pub async fn handle_login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, LoginError> {
    let (username, password) = body.validate().map_err(LoginError::EmptyCredentials)?;

    let auth_service = AuthService::new(state.db.clone(), state.config.jwt_secret.clone());
    let token = auth_service.login(username, password).await?;

    Ok((StatusCode::OK, Json(AuthResponse::with_token(token))))
}
