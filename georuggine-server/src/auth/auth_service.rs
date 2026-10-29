use argon2::{Argon2, PasswordHasher, password_hash::PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use password_hash::phc::PasswordHash;
use sqlx::SqlitePool;

use crate::auth::{
    dto::{Claims, LoginError, RegisterError},
    user_repository::{UserEntity, UserRepository},
};

const TOKEN_DURATION: Duration = Duration::hours(24);

pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(db_pool: SqlitePool, jwt_secret: String) -> Self {
        Self {
            user_repo: UserRepository::new(db_pool),
            jwt_secret,
        }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<u32, RegisterError> {
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes())
            .map_err(|e| RegisterError::Internal(e.to_string()))?
            .to_string();

        self.user_repo
            .create_user(username, &password_hash)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(err) if err.is_unique_violation() => {
                    RegisterError::UsernameAlreadyExists
                }
                other => RegisterError::Internal(other.to_string()),
            })
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String, LoginError> {
        let user: UserEntity = self
            .user_repo
            .find_by_username(username)
            .await
            .map_err(|e| LoginError::Internal(e.to_string()))?
            .ok_or(LoginError::InvalidCredentials)?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|_| LoginError::Internal("Invalid password hash format".to_string()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|e| match e {
                password_hash::Error::PasswordInvalid => LoginError::InvalidCredentials,
                other => LoginError::Internal(other.to_string()),
            })?;

        self.generate_jwt(user.id, username)
            .map_err(|e| LoginError::Internal(format!("Failed to generate token: {e}")))
    }

    pub fn validate_jwt(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    fn generate_jwt(
        &self,
        user_id: u32,
        username: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expire = now + TOKEN_DURATION;

        let claims = Claims {
            user_id,
            username: username.to_string(),
            iat: now.timestamp() as usize,
            exp: expire.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
    }
}
