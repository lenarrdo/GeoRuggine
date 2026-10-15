use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"super0-secretW-fleet0-key";
const TOKEN_DURATION: Duration = Duration::hours(24);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_jwt(user_id: i64, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
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
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(), // automatically validates expiration date
    )?;

    Ok(token_data.claims)
}
