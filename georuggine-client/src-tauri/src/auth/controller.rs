use crate::auth::auth_service::AuthService;
use crate::auth::models::{AuthResult, UserSession};
use crate::state::AppState;
use crate::websocket::connect_ws_internal;

pub struct AuthController;

impl AuthController {
    /// Business logic for user registration
    pub async fn register(username: String, password: String) -> AuthResult {
        match AuthService::register(username, password).await {
            Ok(body) => AuthResult {
                success: true,
                message: body.message,
            },
            Err(err_msg) => AuthResult {
                success: false,
                message: err_msg,
            },
        }
    }

    /// Business logic for user login:
    /// - HTTP authentication through AuthService
    /// - Session (JWT token) storage in the protected Rust state
    /// - WebSocket connection opening with the token
    /// - Background coordinate tracking task startup
    pub async fn login(
        app_handle: &tauri::AppHandle,
        state: &AppState,
        username: String,
        password: String,
    ) -> AuthResult {
        let body = match AuthService::login(username.clone(), password).await {
            Ok(body) => body,
            Err(err_msg) => {
                return AuthResult {
                    success: false,
                    message: err_msg,
                };
            }
        };

        if let Some(token) = &body.token {
            // 1. Connect the WebSocket: without chat the login does not succeed
            if connect_ws_internal(app_handle, state, token).await.is_err() {
                return AuthResult {
                    success: false,
                    message: "Unable to connect. Please try again later.".to_string(),
                };
            }

            // 2. Save the full session (with JWT token) only once the WebSocket is active
            let session = UserSession {
                username: username.clone(),
                token: token.clone(),
            };
            if let Err(e) = state.save_session(session) {
                state.disconnect_ws();
                return AuthResult {
                    success: false,
                    message: format!("Failed to save the session: {e}"),
                };
            }

            // 3. Outcome exposed to React (no token leaked)
            AuthResult {
                success: true,
                message: body.message,
            }
        } else {
            AuthResult {
                success: false,
                message: "Missing token in the server response.".to_string(),
            }
        }
    }

    /// Business logic for logout: disconnects the WebSocket and clears the session
    pub fn logout(state: &AppState) -> Result<(), String> {
        state.disconnect_ws();
        state.clear_session()
    }

    /// Returns the username of the currently logged-in user
    pub fn get_current_user(state: &AppState) -> Result<Option<String>, String> {
        let session = state.get_session()?;
        Ok(session.map(|s| s.username))
    }
}
