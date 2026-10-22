pub mod clients;

use axum::{
    Json,
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    ops::ControlFlow,
    sync::{Arc, mpsc},
};

use crate::messages::{ClientIncomingMessage, TextMessage, message_repository::MessageRepository};
use crate::shared::app_state::AppState;
use crate::tui::AppEvent;
use crate::websocket::clients::ConnectedClient;
use crate::{auth::services::token_service, shared::utils};

#[derive(Deserialize)]
pub struct WsParams {
    token: String,
}

/// Messaggio restituito in caso di errore durante l'apertura di un socket.
#[derive(Debug, Clone, Serialize)]
pub struct WsResponse {
    pub message: String,
}

/// Handler HTTP iniziale per la rotta WebSocket (es. `GET {BASE_URL}/ws`).
pub async fn ws_handler(
    Query(params): Query<WsParams>,
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, (StatusCode, Json<WsResponse>)> {
    let claims = token_service::validate_jwt(&params.token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(WsResponse {
                message: "WebSockets can be only opened by authorized users.".to_string(),
            }),
        )
    })?;

    if state.clients.is_connected(claims.user_id) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(WsResponse {
                message: "Users can open only one WebSocket at a time.".to_string(),
            }),
        ));
    }

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, claims.user_id, state)))
}

/// Gestisce l'intero ciclo di vita di una singola connessione WebSocket attiva.
pub async fn handle_socket(socket: WebSocket, user_id: u32, state: Arc<AppState>) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, rx) = mpsc::channel::<TextMessage>();

    state.clients.add(user_id, ConnectedClient::new(tx));
    utils::send_app_events(
        vec![
            AppEvent::UserConnected(user_id),
            AppEvent::LogMessage(format!("User #{user_id} connected.")),
        ],
        &state.tx,
    )
    .await;

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv() {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    let state_clone = Arc::clone(&state);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            let result = process_message(msg, user_id, Arc::clone(&state_clone)).await;
            if result.is_break() {
                break;
            }
        }
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            if let Err(e) = rv_a {
                eprintln!("Errore send_task per {user_id}: {e:?}");
            }
            recv_task.abort();
        }

        rv_b = (&mut recv_task) => {
            if let Err(e) = rv_b {
                eprintln!("Errore recv_task per {user_id}: {e:?}");
            }
            send_task.abort();
        }
    }

    state.clients.remove(user_id);
    utils::send_app_events(
        vec![
            AppEvent::UserDisconnected(user_id),
            AppEvent::LogMessage(format!("User #{user_id} disconnected.")),
        ],
        &state.tx,
    )
    .await;
}

/// Elabora i singoli messaggi ricevuti dal client via WebSocket.
pub async fn process_message(
    msg: Message,
    user_id: u32,
    state: Arc<AppState>,
) -> ControlFlow<(), ()> {
    let message_repo = MessageRepository::new(state.db.clone());
    match msg {
        Message::Text(t) => match serde_json::from_str::<ClientIncomingMessage>(&t) {
            Ok(ClientIncomingMessage::Position(pos)) => {
                let _err = message_repo.save_position(user_id, &pos).await;

                let msg = format!("Received coordinates {} from User {user_id}.", pos);
                state.tx.send(AppEvent::LogMessage(msg)).await.unwrap();

                state.clients.record_position(user_id, pos);
                let event = if let true = state.clients.is_moving(user_id) {
                    AppEvent::UserMoving(user_id)
                } else {
                    AppEvent::UserStopped(user_id)
                };
                state.tx.send(event).await.unwrap();
            }
            Ok(ClientIncomingMessage::Text(text_msg)) => {
                let log_msg = format!("New message from User #{user_id}: {}", text_msg.contents);
                state.tx.send(AppEvent::LogMessage(log_msg)).await.unwrap();

                let _err = message_repo
                    .save_text_message(user_id, None, &text_msg, &text_msg.timestamp)
                    .await;
            }
            Err(_) => {
                let msg = format!("Received unstructured message from User {user_id}: \"{t}\"");
                state.tx.send(AppEvent::LogMessage(msg)).await.unwrap();
            }
        },
        Message::Close(c) => {
            if let Some(cf) = c {
                let msg = format!(
                    "User #{user_id} requested to close the socket ({}).",
                    cf.reason
                );
                state.tx.send(AppEvent::LogMessage(msg)).await.unwrap();
            } else {
                let msg = format!("{user_id} closed the connection.");
                state.tx.send(AppEvent::LogMessage(msg)).await.unwrap();
            }
            return ControlFlow::Break(());
        }
        _ => {}
    }

    ControlFlow::Continue(())
}
