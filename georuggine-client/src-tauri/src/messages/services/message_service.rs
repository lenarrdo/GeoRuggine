use crate::messages::api::dto::ChatMessage;
use crate::state::AppState;
use crate::websocket::api::dto::ClientIncomingMessage;

pub struct MessageService;

impl MessageService {
    pub fn send_chat(
        state: &AppState,
        contents: String,
        receiver_id: Option<i64>,
    ) -> Result<(), String> {
        let sender_id = {
            let guard = state.session.lock().map_err(|e| e.to_string())?;
            guard.as_ref().map(|s| s.user_id).unwrap_or(1)
        };

        let chat_msg = ChatMessage {
            message_id: None,
            sender_id,
            receiver_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            contents,
        };

        state.send_ws_message(ClientIncomingMessage::Chat(chat_msg))
    }
}
