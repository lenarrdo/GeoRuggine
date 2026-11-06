use futures_util::{SinkExt, StreamExt};
use georuggine_client_lib::messages::chatmessage::dto::TextMessage;
use georuggine_client_lib::messages::dto::ClientIncomingMessage;
use georuggine_client_lib::messages::position::dto::PositionUpdate;
use georuggine_client_lib::state::AppState;
use georuggine_client_lib::websocket::{connect_ws_to, WsEventEmitter};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug)]
enum ServerCommand {
    Text(String),
    Close,
}

struct TestServer {
    url: String,
    received: mpsc::UnboundedReceiver<String>,
    outgoing: mpsc::UnboundedSender<ServerCommand>,
    uri: oneshot::Receiver<String>,
    handle: tokio::task::JoinHandle<()>,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

/// Starts a minimal WebSocket server on an ephemeral port and exposes:
/// - `received`: text frames sent by the client
/// - `outgoing`: text/close commands to send to the client
/// - `uri`: the request URI, used to verify the `?token=` query parameter
async fn start_test_server() -> TestServer {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test WebSocket server");
    let address = listener.local_addr().expect("server address");

    let (received_tx, received) = mpsc::unbounded_channel::<String>();
    let (outgoing, mut outgoing_rx) = mpsc::unbounded_channel::<ServerCommand>();
    let (uri_tx, uri) = oneshot::channel::<String>();

    let handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept connection");
        let mut ws = accept_hdr_async(stream, move |request: &Request, response: Response| {
            let _ = uri_tx.send(request.uri().to_string());
            Ok(response)
        })
        .await
        .expect("complete WebSocket handshake");

        loop {
            tokio::select! {
                incoming = ws.next() => {
                    match incoming {
                        Some(Ok(Message::Text(text))) => {
                            let _ = received_tx.send(text.to_string());
                        }
                        Some(Ok(Message::Close(_))) | None => break,
                        Some(Err(_)) => break,
                        _ => {}
                    }
                }
                command = outgoing_rx.recv() => {
                    match command {
                        Some(ServerCommand::Text(text)) => {
                            if ws.send(Message::Text(text.into())).await.is_err() {
                                break;
                            }
                        }
                        Some(ServerCommand::Close) | None => break,
                    }
                }
            }
        }
    });

    TestServer {
        url: format!("ws://{address}"),
        received,
        outgoing,
        uri,
        handle,
    }
}

#[derive(Clone, Default)]
struct Recorder {
    chats: Arc<Mutex<Vec<TextMessage>>>,
    disconnects: Arc<Mutex<usize>>,
}

impl WsEventEmitter for Recorder {
    fn emit_chat_message(&self, message: TextMessage) {
        self.chats.lock().expect("chats mutex").push(message);
    }

    fn emit_disconnected(&self) {
        *self.disconnects.lock().expect("disconnects mutex") += 1;
    }
}

async fn wait_until(mut predicate: impl FnMut() -> bool) {
    tokio::time::timeout(Duration::from_secs(3), async {
        while !predicate() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("condition was not met before the timeout");
}

#[tokio::test]
async fn connect_registers_the_sender_and_forwards_the_token() {
    let mut server = start_test_server().await;
    let state = AppState::default();
    let recorder = Recorder::default();

    connect_ws_to(&recorder, &state, "jwt-123", &server.url)
        .await
        .expect("WebSocket connection");

    assert!(state.is_ws_connected());

    let uri = tokio::time::timeout(Duration::from_secs(2), &mut server.uri)
        .await
        .expect("handshake should complete")
        .expect("URI should be captured");
    assert!(uri.contains("token=jwt-123"), "unexpected URI: {uri}");

    state.disconnect_ws();
}

#[tokio::test]
async fn chat_messages_are_transmitted_as_json_frames() {
    let mut server = start_test_server().await;
    let state = AppState::default();
    let recorder = Recorder::default();

    connect_ws_to(&recorder, &state, "token", &server.url)
        .await
        .expect("WebSocket connection");

    state
        .send_ws_message(ClientIncomingMessage::Text(TextMessage {
            contents: "hello over the wire".to_string(),
            timestamp: "2026-01-01T10:00:00Z".to_string(),
        }))
        .expect("enqueue chat message");

    let frame = tokio::time::timeout(Duration::from_secs(2), server.received.recv())
        .await
        .expect("server should receive a frame")
        .expect("server channel should stay open");

    let value: serde_json::Value = serde_json::from_str(&frame).expect("valid JSON frame");
    assert_eq!(value["contents"], "hello over the wire");
    assert_eq!(value["timestamp"], "2026-01-01T10:00:00Z");

    state.disconnect_ws();
}

#[tokio::test]
async fn positions_are_transmitted_without_an_empty_timestamp() {
    let mut server = start_test_server().await;
    let state = AppState::default();
    let recorder = Recorder::default();

    connect_ws_to(&recorder, &state, "token", &server.url)
        .await
        .expect("WebSocket connection");

    state
        .send_ws_message(ClientIncomingMessage::Position(PositionUpdate {
            lat: 45.0703,
            lon: 7.6869,
            timestamp: None,
        }))
        .expect("enqueue position");

    let frame = tokio::time::timeout(Duration::from_secs(2), server.received.recv())
        .await
        .expect("server should receive a frame")
        .expect("server channel should stay open");

    let value: serde_json::Value = serde_json::from_str(&frame).expect("valid JSON frame");
    assert_eq!(value["lat"], 45.0703);
    assert_eq!(value["lon"], 7.6869);
    assert!(value.get("timestamp").is_none());

    state.disconnect_ws();
}

#[tokio::test]
async fn messages_from_the_server_are_forwarded_to_the_frontend() {
    let server = start_test_server().await;
    let state = AppState::default();
    let recorder = Recorder::default();

    connect_ws_to(&recorder, &state, "token", &server.url)
        .await
        .expect("WebSocket connection");

    let payload = serde_json::to_string(&TextMessage {
        contents: "from the server".to_string(),
        timestamp: "2026-01-01T10:00:00Z".to_string(),
    })
    .expect("serialize");

    server
        .outgoing
        .send(ServerCommand::Text(payload))
        .expect("server should be running");

    let chats = recorder.chats.clone();
    wait_until(move || !chats.lock().expect("chats mutex").is_empty()).await;

    let chats = recorder.chats.lock().expect("chats mutex");
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].contents, "from the server");
    assert_eq!(chats[0].timestamp, "2026-01-01T10:00:00Z");

    state.disconnect_ws();
}

#[tokio::test]
async fn invalid_server_frames_are_ignored() {
    let server = start_test_server().await;
    let state = AppState::default();
    let recorder = Recorder::default();

    connect_ws_to(&recorder, &state, "token", &server.url)
        .await
        .expect("WebSocket connection");

    server
        .outgoing
        .send(ServerCommand::Text("this is not json".to_string()))
        .expect("server should be running");
    server
        .outgoing
        .send(ServerCommand::Text(
            serde_json::to_string(&TextMessage {
                contents: "valid".to_string(),
                timestamp: "2026-01-01T10:00:00Z".to_string(),
            })
            .expect("serialize"),
        ))
        .expect("server should be running");

    let chats = recorder.chats.clone();
    wait_until(move || !chats.lock().expect("chats mutex").is_empty()).await;

    let chats = recorder.chats.lock().expect("chats mutex");
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].contents, "valid");

    state.disconnect_ws();
}

#[tokio::test]
async fn closing_the_server_emits_disconnect_and_clears_the_sender() {
    let server = start_test_server().await;
    let state = AppState::default();
    let recorder = Recorder::default();

    connect_ws_to(&recorder, &state, "token", &server.url)
        .await
        .expect("WebSocket connection");
    assert!(state.is_ws_connected());

    server
        .outgoing
        .send(ServerCommand::Close)
        .expect("server should be running");

    let disconnects = recorder.disconnects.clone();
    wait_until(move || *disconnects.lock().expect("disconnects mutex") >= 1).await;
    wait_until(|| !state.is_ws_connected()).await;

    assert!(!state.is_ws_connected());
    assert!(state
        .send_ws_message(ClientIncomingMessage::Text(TextMessage {
            contents: "late".to_string(),
            timestamp: "2026-01-01T10:00:00Z".to_string(),
        }))
        .is_err());
}

#[tokio::test]
async fn connection_failure_returns_an_error_and_clears_the_state() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral port");
    let address = listener.local_addr().expect("address");
    drop(listener);

    let state = AppState::default();
    let recorder = Recorder::default();

    let error = connect_ws_to(&recorder, &state, "token", &format!("ws://{address}"))
        .await
        .expect_err("connection should fail");

    assert!(error.contains("WebSocket connection to"));
    assert!(!state.is_ws_connected());
}
