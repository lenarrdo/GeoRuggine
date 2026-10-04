use crate::messages::{ChatMessage, ClientIncomingMessage};
use crate::websocket::api::dto::{Clients, ConnectedClient};
use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{net::SocketAddr, ops::ControlFlow};
use tokio::sync::mpsc;

/// Handler HTTP iniziale per la rotta WebSocket (es. `GET /ws`).
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(clients): State<Clients>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, clients))
}

/// Gestisce l'intero ciclo di vita di una singola connessione WebSocket attiva.
pub async fn handle_socket(socket: WebSocket, who: SocketAddr, clients: Clients) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ChatMessage>();

    {
        let mut guard = clients.write().await;
        guard.insert(
            who,
            ConnectedClient {
                user_id: None,
                tx,
            },
        );
        println!("Client {who} connesso (totale attivi: {})", guard.len());
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            if ws_sender.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    let clients_clone = clients.clone();
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = ws_receiver.next().await {
            cnt += 1;
            if process_message(msg, who, &clients_clone).await.is_break() {
                break;
            }
        }

        cnt
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            if let Err(e) = rv_a {
                eprintln!("Errore send_task per {who}: {e:?}");
            }
            recv_task.abort();
        }

        rv_b = (&mut recv_task) => {
            if let Err(e) = rv_b {
                eprintln!("Errore recv_task per {who}: {e:?}");
            }
            send_task.abort();
        }
    }

    {
        let mut guard = clients.write().await;
        guard.remove(&who);
        println!("Client {who} disconnesso (rimanenti: {})", guard.len());
    }

    println!("Websocket context {who} destroyed");
}

/// Elabora i singoli messaggi ricevuti dal client via WebSocket.
pub async fn process_message(
    msg: Message,
    who: SocketAddr,
    clients: &Clients,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            if let Ok(incoming) = serde_json::from_str::<ClientIncomingMessage>(&t) {
                match incoming {
                    ClientIncomingMessage::Position(pos) => {
                        println!(
                            ">>> [COORDINATE DA CLIENT] Da {who} (User {}): x={}, y={}, time={}",
                            pos.user_id, pos.x_coord, pos.y_coord, pos.timestamp
                        );

                        let mut guard = clients.write().await;
                        if let Some(client) = guard.get_mut(&who) {
                            client.user_id = Some(pos.user_id);
                        }
                    }
                    ClientIncomingMessage::Chat(chat_msg) => {
                        println!(
                            ">>> [MESSAGGIO AL SERVER] Da {who} (User {}): `{}`",
                            chat_msg.sender_id, chat_msg.contents
                        );

                        let mut guard = clients.write().await;
                        if let Some(client) = guard.get_mut(&who) {
                            client.user_id = Some(chat_msg.sender_id);
                        }
                    }
                }
            } else {
                println!(">>> {who} ha inviato testo generico non strutturato: {t:?}");
            }
        }
        Message::Binary(d) => {
            println!(">>> {who} ha inviato {} bytes binari: {d:?}", d.len());
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {who} ha inviato chiusura socket code={} reason=`{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> {who} ha chiuso la connessione");
            }
            return ControlFlow::Break(());
        }
        Message::Pong(v) => {
            println!(">>> {who} ha risposto con pong {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {who} ha inviato ping {v:?}");
        }
    }

    ControlFlow::Continue(())
}
