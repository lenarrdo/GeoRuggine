use crate::messages::api::dto::ChatMessage;
use crate::state::AppState;
use crate::websocket::api::dto::ClientIncomingMessage;

pub struct MessageService;

impl MessageService {
    pub fn send_chat(
        state: &AppState,
        contents: String,
        _receiver_id: Option<i64>,
    ) -> Result<(), String> {
        let chat_msg = ChatMessage {
            timestamp: chrono::Utc::now().to_rfc3339(),
            contents,
        };

        state.send_ws_message(ClientIncomingMessage::Chat(chat_msg))
    }
}
