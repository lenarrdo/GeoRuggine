use crate::messages::chatmessage::controller::ChatMessageController;
use crate::state::AppState;

/// Produttore 1: UI (Asincrono / Event-driven)
/// Invocato quando l'utente scrive un messaggio di chat.
/// Accoda il messaggio nel canale MPSC condiviso in modo thread-safe.
#[tauri::command]
pub fn send_chat_message(
    state: tauri::State<'_, AppState>,
    contents: String,
) -> Result<(), String> {
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return Err("Il messaggio non può essere vuoto.".to_string());
    }
    ChatMessageController::send_chat_message(&state, trimmed.to_string(), receiver_id)
}
