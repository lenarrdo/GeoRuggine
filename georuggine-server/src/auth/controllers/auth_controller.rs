use crate::auth::api::dto::{AuthRequest, AuthResponse};
use crate::auth::services::auth_service::{AuthService, LoginError, RegisterError};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;

pub async fn todo_route() -> &'static str {
    "TODO"
}

pub async fn handle_registration(
    State(db_pool): State<SqlitePool>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthResponse>)> {
    let username = body.username.as_deref().unwrap_or("");
    let password = body.password.as_deref().unwrap_or("");

    match AuthService::register(&db_pool, username, password).await {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(AuthResponse {
                message: "success".to_string(),
                token: None,
            }),
        )),
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
    State(db_pool): State<SqlitePool>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthResponse>)> {
    let username = body.username.as_deref().unwrap_or("");
    let password = body.password.as_deref().unwrap_or("");

    match AuthService::login(&db_pool, username, password).await {
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
