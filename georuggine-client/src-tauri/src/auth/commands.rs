use crate::auth::controller::AuthController;
use crate::auth::models::AuthResult;
use crate::state::AppState;

/// Registrazione utente: validazione dei parametri e invocazione del controller
#[tauri::command]
pub async fn register(username: String, password: String) -> Result<AuthResult, String> {
    let trimmed_username = username.trim();
    if trimmed_username.is_empty() {
        return Ok(AuthResult {
            success: false,
            message: "Lo username non può essere vuoto.".to_string(),
        });
    }

    if password.len() < 4 {
        return Ok(AuthResult {
            success: false,
            message: "La password deve contenere almeno 4 caratteri.".to_string(),
        });
    }

    Ok(AuthController::register(trimmed_username.to_string(), password).await)
}

/// Login utente: validazione dei parametri e invocazione del controller
#[tauri::command]
pub async fn login(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    username: String,
    password: String,
) -> Result<AuthResult, String> {
    let trimmed_username = username.trim();
    if trimmed_username.is_empty() || password.trim().is_empty() {
        return Ok(AuthResult {
            success: false,
            message: "Username e password sono obbligatori.".to_string(),
        });
    }

    Ok(AuthController::login(&app_handle, &state, trimmed_username.to_string(), password).await)
}

/// Logout: disconnette tutti i task e resetta la sessione
#[tauri::command]
pub fn logout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    AuthController::logout(&state)
}

/// Recupera l'utente attualmente loggato
#[tauri::command]
pub fn get_current_user(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, String> {
    AuthController::get_current_user(&state)
}
