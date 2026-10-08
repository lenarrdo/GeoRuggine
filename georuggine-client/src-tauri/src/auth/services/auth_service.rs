use crate::auth::api::dto::{AuthRequest, AuthResponse};

pub const SERVER_HTTP_URL: &str = "http://127.0.0.1:3000/api/v1";

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
            .map_err(|e| format!("Errore di connessione al server: {e}"))?;

        let status = res.status();
        let body = res
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Errore di parsing risposta JSON: {e}"))?;

        if status.is_success() {
            Ok(body)
        } else {
            Err(body.message)
        }
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
            .map_err(|e| format!("Errore di connessione al server: {e}"))?;

        let status = res.status();
        let body = res
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Errore di parsing risposta JSON: {e}"))?;

        if !status.is_success() {
            return Err(body.message);
        }

        Ok(body)
    }
}
