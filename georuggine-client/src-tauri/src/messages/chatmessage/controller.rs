use crate::messages::chatmessage::dto::TextMessage;
use crate::messages::dto::ClientIncomingMessage;
use crate::state::AppState;

pub struct ChatMessageController;

impl ChatMessageController {
    pub fn send_chat_message(state: &AppState, contents: String) -> Result<(), String> {
        let msg = TextMessage {
            contents,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        state.send_ws_message(ClientIncomingMessage::Text(msg))
    }
}

pub type MessageController = ChatMessageController;
