use georuggine_client_lib::messages::{
    chatmessage::dto::TextMessage,
    dto::ClientIncomingMessage,
};
use georuggine_client_lib::state::AppState;
use tokio::sync::mpsc;

/// Creates an `AppState` whose WebSocket sender is registered, as it happens
/// right after a successful `connect_ws_to`. The returned receiver represents
/// the transmission task that would normally send frames to the server.
pub fn connected_state() -> (
    AppState,
    mpsc::UnboundedReceiver<ClientIncomingMessage>,
) {
    let state = AppState::default();
    let (tx, rx) = mpsc::unbounded_channel();

    *state.ws_tx.lock().expect("ws_tx mutex poisoned") = Some(tx);

    (state, rx)
}

pub fn text_message(contents: &str, timestamp: &str) -> ClientIncomingMessage {
    ClientIncomingMessage::Text(TextMessage {
        contents: contents.to_string(),
        timestamp: timestamp.to_string(),
    })
}
