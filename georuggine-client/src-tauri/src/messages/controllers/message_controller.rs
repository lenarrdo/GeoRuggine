use crate::messages::services::message_service::MessageService;
use crate::state::AppState;

/// Produttore 1: UI (Asincrono / Event-driven)
/// Invocato quando l'utente scrive un messaggio di chat.
/// Accoda il messaggio nel canale MPSC condiviso in modo thread-safe.
#[tauri::command]
pub fn send_chat_message(
    state: tauri::State<'_, AppState>,
    contents: String,
    receiver_id: Option<i64>,
) -> Result<(), String> {
    MessageService::send_chat(&state, contents, receiver_id)
}
