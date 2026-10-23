use crate::auth::auth_service::AuthService;
use crate::auth::models::{AuthResult, UserSession};
use crate::state::AppState;
use crate::websocket::connect_ws_internal;

pub struct AuthController;

impl AuthController {
    /// Business logic per la registrazione utente
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

    /// Business logic per il login utente:
    /// - Autenticazione HTTP tramite AuthService
    /// - Salvataggio della sessione (token JWT) nello stato protetto Rust
    /// - Apertura della connessione WebSocket con il token
    /// - Avvio del task di tracking delle coordinate in background
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
            // 1. Salva la sessione completa (con token JWT) nello stato protetto Rust
            let session = UserSession {
                username: username.clone(),
                token: token.clone(),
            };
            if let Err(e) = state.save_session(session) {
                return AuthResult {
                    success: false,
                    message: format!("Errore nel salvataggio della sessione: {e}"),
                };
            }

            // 2. Connette la WebSocket al server usando il token
            if let Err(e) = connect_ws_internal(app_handle, state, token).await {
                eprintln!("[AuthController] Login completato, ma errore apertura WebSocket: {e}");
            }

            // 4. Risultato esposto a React (nessun token trapelato)
            AuthResult {
                success: true,
                message: body.message,
            }
        } else {
            AuthResult {
                success: false,
                message: "Token mancante nella risposta del server.".to_string(),
            }
        }
    }

    /// Business logic per il logout: disconnette WebSocket e azzera la sessione
    pub fn logout(state: &AppState) -> Result<(), String> {
        state.disconnect_ws();
        state.clear_session()
    }

    /// Recupera l'username dell'utente correntemente loggato
    pub fn get_current_user(state: &AppState) -> Result<Option<String>, String> {
        let session = state.get_session()?;
        Ok(session.map(|s| s.username))
    }
}
