use crate::auth::api::dto::AuthResponse;
use crate::auth::domain::session::UserSession;
use crate::auth::repository::session_repository::SessionRepository;
use crate::auth::services::auth_service::AuthService;
use crate::state::AppState;
use crate::tracking::start_background_tracking;
use crate::websocket::connect_ws_internal;

/// Registrazione utente presso georuggine-server
#[tauri::command]
pub async fn register(username: String, password: String) -> Result<AuthResponse, String> {
    AuthService::register(username, password).await
}

/// Login utente: salvataggio sessione, connessione automatica WebSocket
/// e avvio automatico del Task 3 di tracking periodico in background
#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    username: String,
    password: String,
) -> Result<AuthResponse, String> {
    let body = AuthService::login(username.clone(), password).await?;

    if let Some(token) = &body.token {
        // 1. Salva la sessione nello stato condiviso
        let session = UserSession {
            username: username.clone(),
            token: token.clone(),
        };
        SessionRepository::save(&state, session)?;

        // 2. Connette la WebSocket (avvia Task 1 Trasmissione e Task 2 Ricezione)
        if let Err(e) = connect_ws_internal(&app_handle, &state, token).await {
            eprintln!("[Auth] Login completato, ma errore apertura WebSocket: {e}");
        } else {
            // 3. Avvia Task 3 (Tracking periodico da file ogni 30s)
            if let Err(e) = start_background_tracking(&app_handle, &state).await {
                eprintln!("[Auth] Errore avvio background tracking: {e}");
            }
        }
    }

    Ok(body)
}

/// Logout: disconnette tutti i task e resetta la sessione
#[tauri::command]
pub fn logout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.disconnect_ws();
    SessionRepository::clear(&state)
}

/// Recupera l'utente attualmente loggato
#[tauri::command]
pub fn get_current_user(
    state: tauri::State<'_, AppState>,
) -> Result<Option<UserSession>, String> {
    SessionRepository::get(&state)
}
