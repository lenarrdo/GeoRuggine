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
    // Chiudi preventivamente eventuali sessioni aperte
    state.disconnect_ws();

    // 1. Connessione fisica WebSocket al server remoto passando il token JWT
    let ws_endpoint = format!("{WS_URL}?token={token}");
    let (ws_stream, _) = connect_async(&ws_endpoint)
        .await
        .map_err(|e| format!("Connessione WebSocket fallita verso {ws_endpoint}: {e}"))?;

    let (mut ws_sink, mut ws_stream) = ws_stream.split();

    // 2. Creazione del canale MPSC: il buffer condiviso tra produttori e consumatore web
    let (tx, mut rx) = mpsc::unbounded_channel::<ClientIncomingMessage>();

    // Registra tx nello stato condiviso (accessibile da UI e dal Tracking)
    if let Ok(mut tx_guard) = state.ws_tx.lock() {
        *tx_guard = Some(tx.clone());
    }

    // TASK 1: TRASMISSIONE (Unico consumatore del canale sul web)
    let state_for_send = state.clone();
    let app_for_send = app_handle.clone();
    let send_handle = tokio::spawn(async move {
        println!("[WS Client] Task 1 (Trasmissione) avviato");
        while let Some(msg) = rx.recv().await {
            match serde_json::to_string(&msg) {
                Ok(json) => {
                    if let Err(e) = ws_sink.send(Message::Text(json.into())).await {
                        eprintln!("[WS Client] Errore invio messaggio su socket: {e}");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("[WS Client] Errore serializzazione JSON: {e}");
                }
            }
        }
        let _ = ws_sink.close().await;
        println!("[WS Client] Task 1 (Trasmissione) concluso");
        // Chiusura imprevista: avvisa il frontend (non avviene su logout, dove il task viene abortito)
        let _ = app_for_send.emit("ws-disconnected", ());
        state_for_send.disconnect_ws();
    })
    .abort_handle();

    // TASK 2: RICEZIONE (In ascolto continuo dei messaggi dal server)
    let app_clone = app_handle.clone();
    let state_for_recv = state.clone();
    let recv_handle = tokio::spawn(async move {
        println!("[WS Client] Task 2 (Ricezione) avviato");
        while let Some(msg_result) = ws_stream.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    if let Ok(chat) = serde_json::from_str::<TextMessage>(&text) {
                        let _ = app_clone.emit("chat-message-received", chat);
                    } else {
                        eprintln!("[WS Client] Ricevuto messaggio di testo non conforme a TextMessage: {text}");
                    }
                }
                Ok(Message::Close(close_frame)) => {
                    println!("[WS Client] Socket chiuso dal server: {close_frame:?}");
                    break;
                }
                Ok(Message::Ping(_)) => {}
                Err(e) => {
                    eprintln!("[WS Client] Errore ricezione socket: {e}");
                    break;
                }
                _ => {}
            }
        }

        println!("[WS Client] Task 2 (Ricezione) concluso");
        // Chiusura imprevista: avvisa il frontend (non avviene su logout, dove il task viene abortito)
        let _ = app_clone.emit("ws-disconnected", ());
        state_for_recv.disconnect_ws();
    })
    .abort_handle();

    // Salva gli handle per consentire al Main Thread di abortire alla chiusura della finestra
    if let Ok(mut guard) = state.send_abort.lock() {
        *guard = Some(send_handle);
    }
    if let Ok(mut guard) = state.recv_abort.lock() {
        *guard = Some(recv_handle);
    }

    Ok(())
}
