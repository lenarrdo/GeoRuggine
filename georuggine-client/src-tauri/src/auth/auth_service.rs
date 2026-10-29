use crate::auth::models::dto::{AuthRequest, AuthResponse};

pub const SERVER_HTTP_URL: &str = "http://127.0.0.1:3000/api/v1";

fn handle_reqwest_error(e: reqwest::Error) -> String {
    if e.is_connect() {
        "Unable to connect to the server. Please try again later.".to_string()
    } else if e.is_timeout() {
        "The server did not respond in time. Please try again later.".to_string()
    } else {
        "Communication error with the server. Please try again later.".to_string()
    }
}

pub struct AuthService;

impl AuthService {
    pub async fn register(username: String, password: String) -> Result<AuthResponse, String> {
        let client = reqwest::Client::new();
        let url = format!("{SERVER_HTTP_URL}/auth/register");
        let payload = AuthRequest { username, password };

        let res = client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(handle_reqwest_error)?;

        let status = res.status();
        let body = res
            .json::<AuthResponse>()
            .await
            .map_err(|_| "Invalid response received from the server.".to_string())?;

        if !status.is_success() {
            return Err(body.message);
        }

        Ok(body)
    }

    pub async fn login(username: String, password: String) -> Result<AuthResponse, String> {
        let client = reqwest::Client::new();
        let url = format!("{SERVER_HTTP_URL}/auth/login");
        let payload = AuthRequest {
            username,
            password,
        };

        let res = client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(handle_reqwest_error)?;

        let status = res.status();
        let body = res
            .json::<AuthResponse>()
            .await
            .map_err(|_| "Invalid response received from the server.".to_string())?;

        if !status.is_success() {
            return Err(body.message);
        }

        Ok(body)
    }
}
