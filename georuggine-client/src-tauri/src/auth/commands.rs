use crate::auth::controller::AuthController;
use crate::auth::models::AuthResult;
use crate::state::AppState;

/// User registration: parameter validation and controller invocation
#[tauri::command]
pub async fn register(username: String, password: String) -> Result<AuthResult, String> {
    let trimmed_username = username.trim();
    if trimmed_username.is_empty() {
        return Ok(AuthResult {
            success: false,
            message: "Username cannot be empty.".to_string(),
        });
    }

    if password.len() < 4 {
        return Ok(AuthResult {
            success: false,
            message: "Password must be at least 4 characters long.".to_string(),
        });
    }

    Ok(AuthController::register(trimmed_username.to_string(), password).await)
}

/// User login: parameter validation and controller invocation
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
            message: "Username and password are required.".to_string(),
        });
    }

    Ok(AuthController::login(&app_handle, &state, trimmed_username.to_string(), password).await)
}

/// Logout: disconnects every task and resets the session
#[tauri::command]
pub fn logout(state: tauri::State<'_, AppState>) -> Result<(), String> {
    AuthController::logout(&state)
}

/// Returns the currently logged-in user
#[tauri::command]
pub fn get_current_user(
    state: tauri::State<'_, AppState>,
) -> Result<Option<String>, String> {
    AuthController::get_current_user(&state)
}
