use crate::auth::{repository::user_repository::UserRepository, services::token_service};
use argon2::{Argon2, PasswordHasher, password_hash::PasswordVerifier};
use password_hash::phc::PasswordHash;
use sqlx::SqlitePool;

#[allow(dead_code)]
pub enum RegisterError {
    EmptyCredentials,
    UsernameExists,
    HashError,
    DatabaseError(String),
}

#[allow(dead_code)]
pub enum LoginError {
    EmptyCredentials,
    InvalidCredentials,
    InternalError(String),
}

pub struct AuthService;

impl AuthService {
    pub async fn register(
        db_pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<(), RegisterError> {
        let username = username.trim();
        let password = password.trim();

        if username.is_empty() || password.is_empty() {
            return Err(RegisterError::EmptyCredentials);
        }

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes())
            .map_err(|_| RegisterError::HashError)?
            .to_string();

        match UserRepository::create_user(db_pool, username, &password_hash).await {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(RegisterError::UsernameExists)
            }
            Err(e) => {
                eprintln!("Database error: {e}");
                Err(RegisterError::DatabaseError(e.to_string()))
            }
        }
    }

    pub async fn login(
        db_pool: &SqlitePool,
        username: &str,
        password: &str,
    ) -> Result<String, LoginError> {
        let username = username.trim();
        let password = password.trim();

        if username.is_empty() || password.is_empty() {
            return Err(LoginError::EmptyCredentials);
        }

        let user = UserRepository::find_by_username(db_pool, username)
            .await
            .map_err(|e| {
                eprintln!("Errore DB: {e}");
                LoginError::InvalidCredentials
            })?
            .ok_or(LoginError::InvalidCredentials)?;

        let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
            eprintln!("Invalid password hash: {e}");
            LoginError::InternalError("Invalid password hash".to_string())
        })?;

        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => {
                let token = token_service::generate_jwt(user.id, username).map_err(|_| {
                    LoginError::InternalError("Couldn't Generate Token.".to_string())
                })?;
                Ok(token)
            }
            Err(password_hash::Error::PasswordInvalid) => Err(LoginError::InvalidCredentials),
            Err(e) => Err(LoginError::InternalError(e.to_string())),
        }
    }
}
