pub mod clients;

use axum::{
    Json,
    extract::{
        Query, State,
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{net::SocketAddr, ops::ControlFlow};
use tokio::sync::mpsc::{self, Sender};

use crate::tui::AppEvent;
use crate::websocket::clients::{ClientsList, ConnectedClient};
use crate::{auth::services::token_service, tui::user_list::User};
use crate::{
    messages::{ClientIncomingMessage, TextMessage},
    tracking::Coordinates,
};

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
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(clients): State<Arc<ClientsList>>,
    State(tx): State<Sender<AppEvent>>,
) -> Result<impl IntoResponse, (StatusCode, Json<WsResponse>)> {
    let claims = token_service::validate_jwt(&params.token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(WsResponse {
                message: "WebSockets can be only opened by authorized users.".to_string(),
            }),
        )
    })?;

    if clients.is_connected(claims.user_id).await {
        return Err((
            StatusCode::FORBIDDEN,
            Json(WsResponse {
                message: "Users can open only one WebSocket at a time.".to_string(),
            }),
        ));
    }

    let uname = claims.username.clone();
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, claims.user_id, uname, clients, tx)))
}

/// Gestisce l'intero ciclo di vita di una singola connessione WebSocket attiva.
pub async fn handle_socket(
    socket: WebSocket,
    user_id: u32,
    username: String,
    clients: Arc<ClientsList>,
    tui_tx: Sender<AppEvent>,
) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<TextMessage>();

    clients.add(user_id, ConnectedClient::new(tx)).await;
    tui_tx
        .send(AppEvent::ClientConnected(User::new(user_id, &username)))
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

    let clients_copy = Arc::clone(&clients);
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = ws_receiver.next().await {
            cnt += 1;
            let result = process_message(msg, user_id).await;
            if result.is_break() {
                break;
            } else if let Some(pos) = result.continue_value().unwrap() {
                clients_copy.record_position(user_id, pos).await;
            }
        }

        cnt
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

    clients.remove(user_id).await;
    tui_tx
        .send(AppEvent::ClientDisconnected(user_id))
        .await
        .unwrap();
}

/// Elabora i singoli messaggi ricevuti dal client via WebSocket.
pub async fn process_message(msg: Message, user_id: u32) -> ControlFlow<(), Option<Coordinates>> {
    let mut new_position = None;
    match msg {
        Message::Text(t) => {
            if let Ok(incoming) = serde_json::from_str::<ClientIncomingMessage>(&t) {
                match incoming {
                    ClientIncomingMessage::Position(pos) => {
                        new_position = Some(Coordinates::new(pos.x_coord, pos.y_coord));
                        // println!(
                        //     ">>> [COORDINATE DA CLIENT] Da user_id {user_id}: x={}, y={}, time={}",
                        //     pos.x_coord, pos.y_coord, pos.timestamp
                        // );
                    }
                    ClientIncomingMessage::Text(chat_msg) => {
                        println!(
                            ">>> [MESSAGGIO AL SERVER] Da user_id {user_id} (User {}): `{}`",
                            chat_msg.sender_id, chat_msg.contents
                        );
                    }
                }
            } else {
                println!(">>> {user_id} ha inviato testo generico non strutturato: {t:?}");
            }
        }
        Message::Binary(d) => {
            println!(">>> {user_id} ha inviato {} bytes binari: {d:?}", d.len());
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {user_id} ha inviato chiusura socket code={} reason=`{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> {user_id} ha chiuso la connessione");
            }
            return ControlFlow::Break(());
        }
        Message::Pong(v) => {
            println!(">>> {user_id} ha risposto con pong {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {user_id} ha inviato ping {v:?}");
        }
    }

    ControlFlow::Continue(new_position)
}
