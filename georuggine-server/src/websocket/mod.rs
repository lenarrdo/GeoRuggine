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
use std::ops::ControlFlow;
use std::sync::Arc;
use tokio::sync::mpsc::{self};

use crate::auth::services::token_service;
use crate::messages::{
    ClientIncomingMessage, Coordinates, TextMessage, message_repository::MessageRepository,
};
use crate::shared::app_state::AppState;
use crate::tui::AppEvent;
use crate::websocket::clients::ConnectedClient;

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

    if state.clients.is_connected(claims.user_id).await {
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
    let (tx, mut rx) = mpsc::unbounded_channel::<TextMessage>();

    state.clients.add(user_id, ConnectedClient::new(tx)).await;
    state
        .tx
        .send(AppEvent::ClientConnected(user_id))
        .await
        .unwrap();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    let state_clone = Arc::clone(&state);
    let mut recv_task = tokio::spawn(async move {
        let mut nof_processed_messages = 0;

        let message_repo = MessageRepository::new(state_clone.db.clone());
        while let Some(Ok(msg)) = ws_receiver.next().await {
            let result = process_message(msg, user_id, &message_repo).await;
            if result.is_break() {
                break;
            } else if let Some(pos) = result.continue_value().unwrap() {
                state_clone.clients.record_position(user_id, pos).await;
                let event = if let true = state_clone.clients.is_moving(user_id).await {
                    AppEvent::UserMoving(user_id)
                } else {
                    AppEvent::UserStopped(user_id)
                };

                state_clone.tx.send(event).await.unwrap();
            }

            nof_processed_messages += 1;
        }

        nof_processed_messages
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

    state.clients.remove(user_id).await;
    state
        .tx
        .send(AppEvent::ClientDisconnected(user_id))
        .await
        .unwrap();
}

/// Elabora i singoli messaggi ricevuti dal client via WebSocket.
pub async fn process_message(
    msg: Message,
    user_id: u32,
    message_repo: &MessageRepository,
) -> ControlFlow<(), Option<Coordinates>> {
    let mut new_position = None;
    match msg {
        Message::Text(t) => {
            match serde_json::from_str::<ClientIncomingMessage>(&t) {
                Ok(ClientIncomingMessage::Position(pos)) => {
                    if let Err(_e) = message_repo.save_position(user_id, &pos).await {
                        // eprintln!("Errore salvataggio posizione nel DB per user_id {user_id}: {e}");
                    }

                    // println!(
                    //     ">>> [COORDINATE DA CLIENT] Da user_id {user_id}: x={}, y={}, time={timestamp}",
                    //     pos.lat, pos.lon
                    // );

                    new_position = Some(pos);
                }
                Ok(ClientIncomingMessage::Text(chat_msg)) => {
                    // println!(
                    //     ">>> [MESSAGGIO AL SERVER] Da user_id {user_id} ({}): `{}`",
                    //     chat_msg.timestamp, chat_msg.contents
                    // );

                    // Il server associa sender_id = user_id, receiver_id = None (Centrale/Server) e timestamp fornito dal client
                    if let Err(_e) = message_repo
                        .save_text_message(user_id, None, &chat_msg, &chat_msg.timestamp)
                        .await
                    {
                        // eprintln!("Errore salvataggio messaggio nel DB per user_id {user_id}: {e}");
                    }
                }
                Err(_) => {
                    // println!(">>> {user_id} ha inviato testo generico non strutturato: {t:?}")
                }
            }
        }
        Message::Close(c) => {
            if let Some(_cf) = c {
                // println!(
                //     ">>> {user_id} ha inviato chiusura socket code={} reason=`{}`",
                //     cf.code, cf.reason
                // );
            } else {
                // println!(">>> {user_id} ha chiuso la connessione");
            }
            return ControlFlow::Break(());
        }
        _other => {
            // println!(">>> {user_id} ha inviato un messaggio di servizio/altro: {other:?}");
        }
    }

    ControlFlow::Continue(new_position)
}
