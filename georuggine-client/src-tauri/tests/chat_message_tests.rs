mod common;

use chrono::DateTime;
use georuggine_client_lib::messages::chatmessage::controller::ChatMessageController;
use georuggine_client_lib::messages::chatmessage::dto::TextMessage;
use georuggine_client_lib::messages::dto::ClientIncomingMessage;
use georuggine_client_lib::state::AppState;

#[test]
fn chat_message_is_enqueued_with_contents_and_timestamp() {
    let (state, mut rx) = common::connected_state();

    ChatMessageController::send_chat_message(&state, "hello".to_string())
        .expect("chat message should be enqueued");

    match rx.try_recv().expect("queued message") {
        ClientIncomingMessage::Text(message) => {
            assert_eq!(message.contents, "hello");
            let timestamp = DateTime::parse_from_rfc3339(&message.timestamp)
                .expect("timestamp must be RFC 3339");
            assert!(timestamp.timestamp() > 0);
        }
        ClientIncomingMessage::Position(_) => panic!("expected a text message"),
    }
}

#[test]
fn chat_message_preserves_the_given_contents() {
    let (state, mut rx) = common::connected_state();

    ChatMessageController::send_chat_message(&state, "  spaced out  ".to_string())
        .expect("chat message should be enqueued");

    match rx.try_recv().expect("queued message") {
        ClientIncomingMessage::Text(message) => {
            assert_eq!(message.contents, "  spaced out  ");
        }
        ClientIncomingMessage::Position(_) => panic!("expected a text message"),
    }
}

#[test]
fn chat_message_fails_when_the_websocket_is_not_connected() {
    let state = AppState::default();

    let error = ChatMessageController::send_chat_message(&state, "hello".to_string())
        .expect_err("send should fail");

    assert_eq!(error, "WebSocket is not connected to the server");
    assert!(!state.is_ws_connected());
}

#[test]
fn multiple_chat_messages_keep_their_order() {
    let (state, mut rx) = common::connected_state();

    for contents in ["first", "second", "third"] {
        ChatMessageController::send_chat_message(&state, contents.to_string())
            .expect("chat message should be enqueued");
    }

    let received: Vec<String> = std::iter::from_fn(|| rx.try_recv().ok())
        .map(|message| match message {
            ClientIncomingMessage::Text(message) => message.contents,
            ClientIncomingMessage::Position(_) => panic!("expected only text messages"),
        })
        .collect();

    assert_eq!(received, vec!["first", "second", "third"]);
}

#[test]
fn text_message_serializes_to_the_expected_wire_format() {
    let message = TextMessage {
        contents: "hello".to_string(),
        timestamp: "2026-01-01T10:00:00Z".to_string(),
    };

    let json = serde_json::to_value(&message).expect("serialize");

    assert_eq!(json["contents"], "hello");
    assert_eq!(json["timestamp"], "2026-01-01T10:00:00Z");
    assert_eq!(json.as_object().expect("object").len(), 2);
}

#[test]
fn incoming_text_message_can_be_deserialized() {
    let message: TextMessage =
        serde_json::from_str(r#"{"contents":"hi","timestamp":"2026-01-01T10:00:00Z"}"#)
            .expect("deserialize");

    assert_eq!(message.contents, "hi");
    assert_eq!(message.timestamp, "2026-01-01T10:00:00Z");
}
