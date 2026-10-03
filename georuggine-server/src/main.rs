mod database;

use crate::database as db;

use argon2::{Argon2, PasswordHasher, password_hash::PasswordVerifier};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use password_hash::phc::PasswordHash;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

const HOST: &str = "127.0.0.1";
const PORT: &str = "3000";
const BASE_URL: &str = "/api/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db::init(true).await?;

    let app = Router::new()
        .route(
            &format!("{BASE_URL}/auth/register"),
            post(handle_registration),
        )
        .route(&format!("{BASE_URL}/auth/login"), post(handle_login))
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
    pub token: Option<String>,
}

async fn handle_registration(
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
                token: None,
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
                token: None,
            }),
        ));
    }

    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?);",
        username,
        &password_hash.unwrap().to_string(),
    )
    .execute(&db_pool)
    .await;

    match result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(AuthResponse {
                message: "success".to_string(),
                token: None,
            }),
        )),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => Err((
            StatusCode::CONFLICT,
            Json(AuthResponse {
                message: "Username already exists.".to_string(),
                token: None,
            }),
        )),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthResponse {
                    message: "Failed to register user.".to_string(),
                    token: None,
                }),
            ))
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

async fn handle_login(
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
                token: None,
            }),
        ));
    }

    let user = sqlx::query_as!(User, "SELECT id, username, password_hash FROM users")
        .fetch_optional(&db_pool)
        .await
        .map_err(|e| {
            eprintln!("Errore DB: {}", e);
            (
                StatusCode::NOT_FOUND,
                Json(AuthResponse {
                    message: "Invalid username or password.".to_string(),
                    token: None,
                }),
            )
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(AuthResponse {
                    message: "Credenziali non valide.".to_string(),
                    token: None,
                }),
            ));
        }
    };

    let password_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        eprintln!("Invalid password hash: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: "Internal Server Error".to_string(),
                token: None,
            }),
        )
    })?;

    match Argon2::default().verify_password(password.as_bytes(), &password_hash) {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(AuthResponse {
                message: "success".to_string(),
                token: Some("MOCK_JWT_TOKEN".to_string()),
            }),
        )),
        Err(password_hash::Error::PasswordInvalid) => Err((
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                message: "Invalid username or password".to_string(),
                token: None,
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AuthResponse {
                message: "Internal Server Error".to_string(),
                token: None,
            }),
        )),
    }
}
