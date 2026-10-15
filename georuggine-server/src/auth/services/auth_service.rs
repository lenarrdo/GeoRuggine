use crate::auth::{services::token_service, user_repository::UserRepository};
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

        UserRepository::create_user(db_pool, username, &password_hash)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(e) if e.is_unique_violation() => {
                    RegisterError::UsernameExists
                }
                other => RegisterError::DatabaseError(other.to_string()),
            })
            .map(|_| ())
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
            .map_err(|e| LoginError::InternalError(e.to_string()))?
            .ok_or(LoginError::InvalidCredentials)?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| LoginError::InternalError("Invalid password hash".to_string()))?;

        let _ = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                password_hash::Error::PasswordInvalid => LoginError::InvalidCredentials,
                other => LoginError::InternalError(other.to_string()),
            })
            .map(|_| ())?;

        let token = token_service::generate_jwt(user.id, username)
            .map_err(|_| LoginError::InternalError("Couldn't Generate Token.".to_string()))?;

        Ok(token)
    }
}
