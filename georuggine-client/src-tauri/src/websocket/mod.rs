use crate::messages::{ClientIncomingMessage, TextMessage};
use crate::state::AppState;
use futures_util::{SinkExt, StreamExt};
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub const WS_URL: &str = "ws://127.0.0.1:3000/api/v1/ws";

pub async fn connect_ws_internal(
    app_handle: &tauri::AppHandle,
    state: &AppState,
    token: &str,
) -> Result<(), String> {
    // Proactively close any open session
    state.disconnect_ws();

    // 1. Physical WebSocket connection to the remote server, passing the JWT token
    let ws_endpoint = format!("{WS_URL}?token={token}");
    let (ws_stream, _) = connect_async(&ws_endpoint)
        .await
        .map_err(|e| format!("WebSocket connection to {ws_endpoint} failed: {e}"))?;

    let (mut ws_sink, mut ws_stream) = ws_stream.split();

    // 2. MPSC channel creation: the buffer shared between producers and the web consumer
    let (tx, mut rx) = mpsc::unbounded_channel::<ClientIncomingMessage>();

    // Register tx in the shared state (reachable by the UI and by Tracking)
    if let Ok(mut tx_guard) = state.ws_tx.lock() {
        *tx_guard = Some(tx.clone());
    }

    // TASK 1: TRANSMISSION (the only consumer of the channel towards the web)
    let state_for_send = state.clone();
    let app_for_send = app_handle.clone();
    let send_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if ws_sink.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
        let _ = ws_sink.close().await;
        // Unexpected close: notify the frontend (does not happen on logout, where the task is aborted)
        let _ = app_for_send.emit("ws-disconnected", ());
        state_for_send.disconnect_ws();
    })
    .abort_handle();

    // TASK 2: RECEPTION (continuously listening for messages from the server)
    let app_clone = app_handle.clone();
    let state_for_recv = state.clone();
    let recv_handle = tokio::spawn(async move {
        while let Some(msg_result) = ws_stream.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    if let Ok(chat) = serde_json::from_str::<TextMessage>(&text) {
                        let _ = app_clone.emit("chat-message-received", chat);
                    }
                }
                Ok(Message::Close(_)) => break,
                Ok(Message::Ping(_)) => {}
                Err(_) => break,
                _ => {}
            }
        }

        // Unexpected close: notify the frontend (does not happen on logout, where the task is aborted)
        let _ = app_clone.emit("ws-disconnected", ());
        state_for_recv.disconnect_ws();
    })
    .abort_handle();

    // Store the handles so the Main Thread can abort them when the window closes
    if let Ok(mut guard) = state.send_abort.lock() {
        *guard = Some(send_handle);
    }
    if let Ok(mut guard) = state.recv_abort.lock() {
        *guard = Some(recv_handle);
    }

    Ok(())
}
