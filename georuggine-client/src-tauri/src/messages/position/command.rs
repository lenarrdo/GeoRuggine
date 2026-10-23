use crate::messages::position::controller::PositionController;
use crate::state::AppState;

#[tauri::command]
pub async fn upload_coordinates(
    state: tauri::State<'_, AppState>,
    content: String,
) -> Result<(), String> {

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err("Il contenuto del file è vuoto.".to_string());
    }
    PositionController::upload_coordinates(&state, trimmed).await
}
