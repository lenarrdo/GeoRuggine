use georuggine_client_lib::messages::{
    chatmessage::dto::TextMessage,
    dto::ClientIncomingMessage,
};
use georuggine_client_lib::state::AppState;
use std::thread;
use tokio::sync::mpsc;

fn connected_state() -> (AppState, mpsc::UnboundedReceiver<ClientIncomingMessage>) {
    let state = AppState::default();
    let (tx, rx) = mpsc::unbounded_channel();

    *state.ws_tx.lock().expect("ws_tx mutex poisoned") = Some(tx);

    (state, rx)
}

#[test]
fn send_fails_when_websocket_is_disconnected() {
    let state = AppState::default();
    let message = ClientIncomingMessage::Text(TextMessage {
        contents: "hello".to_string(),
        timestamp: "2026-01-01T00:00:00Z".to_string(),
    });

    let result = state.send_ws_message(message);

    assert!(result.is_err());
    assert!(!state.is_ws_connected());
}

#[test]
fn send_ws_message_enqueues_chat_message() {
    let (state, mut rx) = connected_state();
    let contents = "hello from test";

    state
        .send_ws_message(ClientIncomingMessage::Text(TextMessage {
            contents: contents.to_string(),
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }))
        .expect("message should be enqueued");

    match rx.try_recv().expect("queued message should be available") {
        ClientIncomingMessage::Text(message) => assert_eq!(message.contents, contents),
        ClientIncomingMessage::Position(_) => panic!("expected a chat message"),
    }
}

#[test]
fn concurrent_producers_enqueue_all_messages() {
    const PRODUCERS: usize = 8;
    const MESSAGES_PER_PRODUCER: usize = 100;

    let (state, mut rx) = connected_state();
    let producers: Vec<_> = (0..PRODUCERS)
        .map(|producer_id| {
            let state = state.clone();
            thread::spawn(move || {
                for message_id in 0..MESSAGES_PER_PRODUCER {
                    state
                        .send_ws_message(ClientIncomingMessage::Text(TextMessage {
                            contents: format!("{producer_id}:{message_id}"),
                            timestamp: "2026-01-01T00:00:00Z".to_string(),
                        }))
                        .expect("message should be enqueued");
                }
            })
        })
        .collect();

    for producer in producers {
        producer.join().expect("producer thread should not panic");
    }

    let received: Vec<_> = std::iter::from_fn(|| rx.try_recv().ok()).collect();
    assert_eq!(received.len(), PRODUCERS * MESSAGES_PER_PRODUCER);

    let received_contents: std::collections::HashSet<_> = received
        .into_iter()
        .map(|message| match message {
            ClientIncomingMessage::Text(message) => message.contents,
            ClientIncomingMessage::Position(_) => panic!("expected only chat messages"),
        })
        .collect();
    assert_eq!(received_contents.len(), PRODUCERS * MESSAGES_PER_PRODUCER);
}
