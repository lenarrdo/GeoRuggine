mod database;

use crate::database as db;

use argon2::{Argon2, password_hash::PasswordHasher};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

const HOST: &str = "127.0.0.1";
const PORT: &str = "3000";
const BASE_URL: &str = "/api/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db::init(true).await?;

    let app = Router::new()
        .route(&format!("{BASE_URL}/auth/register"), post(register_account))
        .route(&format!("{BASE_URL}/auth/login"), post(todo_route))
        .route(&format!("{BASE_URL}/auth/session"), get(todo_route))
        .route(&format!("{BASE_URL}/auth/session"), delete(todo_route))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn todo_route() -> &'static str {
    "TODO"
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
}

async fn register_account(
    State(db_pool): State<SqlitePool>,
    Json(body): Json<AuthRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<AuthResponse>)> {
    let username = body.username.as_deref().unwrap_or("").trim();
    let password = body.password.as_deref().unwrap_or("").trim();
    if username.is_empty() || password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(AuthResponse {
                message: "Both username and password are required.".to_string(),
            }),
        ));
    }

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes());
    if password_hash.is_err() {
        eprintln!("Couldn't hash password.");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: "Internal Server Error.".to_string(),
            }),
        ));
    }

    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?);",
        &body.username,
        &password_hash.unwrap().to_string(),
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(AuthResponse {
                message: "success".to_string(),
            }),
        )),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => Err((
            StatusCode::CONFLICT,
            Json(AuthResponse {
                message: "Username already exists.".to_string(),
            }),
        )),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    message: "Failed to register user.".to_string(),
                }),
            ))
        }
    }
}
