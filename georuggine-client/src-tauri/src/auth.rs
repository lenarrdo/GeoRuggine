use crate::models::{AuthRequest, AuthResponse, UserSession};
use crate::state::AppState;
use crate::tracking::start_background_tracking;
use crate::ws::connect_ws_internal;

pub const SERVER_HTTP_URL: &str = "http://127.0.0.1:3000/api/v1";

/// Registrazione utente presso georuggine-server
#[tauri::command]
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

/// Login utente: salvataggio sessione, connessione automatica WebSocket
/// e avvio automatico del Task 3 di tracking periodico in background
#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    username: String,
    password: String,
) -> Result<AuthResponse, String> {
    let client = reqwest::Client::new();
    let url = format!("{SERVER_HTTP_URL}/auth/login");
    let payload = AuthRequest {
        username: username.clone(),
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

    if let Some(token) = &body.token {
        let user_id = 1; // Default id in attesa di estrazione da claim JWT

        // 1. Salva la sessione nello stato condiviso
        if let Ok(mut session_guard) = state.session.lock() {
            *session_guard = Some(UserSession {
                user_id,
                username: username.clone(),
                token: token.clone(),
            });
        }

        // 2. Connette la WebSocket (avvia Task 1 Trasmissione e Task 2 Ricezione)
        if let Err(e) = connect_ws_internal(&app_handle, &state, token).await {
            eprintln!("[Auth] Login completato, ma errore apertura WebSocket: {e}");
        } else {
            // 3. Avvia Task 3 (Tracking periodico da file ogni 30s)
            if let Err(e) = start_background_tracking(&app_handle, &state, user_id).await {
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

    if let Ok(mut session_guard) = state.session.lock() {
        *session_guard = None;
    }

    Ok(())
}

/// Recupera l'utente attualmente loggato
#[tauri::command]
pub fn get_current_user(
    state: tauri::State<'_, AppState>,
) -> Result<Option<UserSession>, String> {
    let session_guard = state.session.lock().map_err(|e| e.to_string())?;
    Ok(session_guard.clone())
}
