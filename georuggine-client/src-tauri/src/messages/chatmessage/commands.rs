use crate::messages::chatmessage::controller::ChatMessageController;
use crate::state::AppState;

/// Producer 1: UI (Asynchronous / Event-driven)
/// Invoked when the user writes a chat message.
/// Enqueues the message on the shared MPSC channel in a thread-safe way.
#[tauri::command]
pub fn send_chat_message(
    state: tauri::State<'_, AppState>,
    contents: String,
) -> Result<(), String> {
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return Err("Message cannot be empty.".to_string());
    }
    ChatMessageController::send_chat_message(&state, trimmed.to_string())
}
