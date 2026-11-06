mod common;

use georuggine_client_lib::auth::UserSession;
use georuggine_client_lib::messages::dto::ClientIncomingMessage;
use georuggine_client_lib::messages::position::dto::PositionUpdate;
use georuggine_client_lib::state::AppState;

#[test]
fn new_state_has_no_session_and_no_connection() {
    let state = AppState::default();

    assert!(state.get_session().expect("session read").is_none());
    assert!(!state.is_ws_connected());
}

#[test]
fn session_can_be_saved_read_and_cleared() {
    let state = AppState::default();
    let session = UserSession {
        username: "alice".to_string(),
        token: "jwt-token".to_string(),
    };

    state.save_session(session).expect("session save");

    let stored = state.get_session().expect("session read").expect("session");
    assert_eq!(stored.username, "alice");
    assert_eq!(stored.token, "jwt-token");

    state.clear_session().expect("session clear");
    assert!(state.get_session().expect("session read").is_none());
}

#[test]
fn session_is_shared_between_cloned_states() {
    let state = AppState::default();
    let clone = state.clone();

    clone
        .save_session(UserSession {
            username: "bob".to_string(),
            token: "token".to_string(),
        })
        .expect("session save on clone");

    let stored = state.get_session().expect("session read").expect("session");
    assert_eq!(stored.username, "bob");

    state.clear_session().expect("session clear on original");
    assert!(clone.get_session().expect("session read").is_none());
}

#[test]
fn sending_without_a_connection_returns_an_error() {
    let state = AppState::default();

    let error = state
        .send_ws_message(common::text_message("hello", "2026-01-01T00:00:00Z"))
        .expect_err("send must fail when disconnected");

    assert_eq!(error, "WebSocket is not connected to the server");
}

#[test]
fn sending_with_a_connection_enqueues_the_message() {
    let (state, mut rx) = common::connected_state();

    assert!(state.is_ws_connected());
    state
        .send_ws_message(common::text_message("hello", "2026-01-01T00:00:00Z"))
        .expect("send");

    match rx.try_recv().expect("queued message") {
        ClientIncomingMessage::Text(message) => assert_eq!(message.contents, "hello"),
        ClientIncomingMessage::Position(_) => panic!("expected a text message"),
    }
}

#[test]
fn sending_a_position_enqueues_the_position_variant() {
    let (state, mut rx) = common::connected_state();

    state
        .send_ws_message(ClientIncomingMessage::Position(PositionUpdate {
            lat: 45.0703,
            lon: 7.6869,
            timestamp: None,
        }))
        .expect("send");

    match rx.try_recv().expect("queued message") {
        ClientIncomingMessage::Position(position) => {
            assert_eq!(position.lat, 45.0703);
            assert_eq!(position.lon, 7.6869);
            assert!(position.timestamp.is_none());
        }
        ClientIncomingMessage::Text(_) => panic!("expected a position message"),
    }
}

#[test]
fn disconnect_drops_the_sender_and_rejects_further_messages() {
    let (state, mut rx) = common::connected_state();

    state.disconnect_ws();

    assert!(!state.is_ws_connected());
    assert!(rx.try_recv().is_err());
    assert!(state
        .send_ws_message(common::text_message("late", "2026-01-01T00:00:00Z"))
        .is_err());
}

#[tokio::test]
async fn disconnect_aborts_every_registered_task() {
    let state = AppState::default();

    let tracking = tokio::spawn(std::future::pending::<()>());
    let sender = tokio::spawn(std::future::pending::<()>());
    let receiver = tokio::spawn(std::future::pending::<()>());

    *state.tracking_abort.lock().expect("tracking abort mutex") =
        Some(tracking.abort_handle());
    *state.send_abort.lock().expect("send abort mutex") = Some(sender.abort_handle());
    *state.recv_abort.lock().expect("recv abort mutex") = Some(receiver.abort_handle());

    state.disconnect_ws();

    for result in [tracking.await, sender.await, receiver.await] {
        let join_error = result.expect_err("task should have been aborted");
        assert!(join_error.is_cancelled());
    }

    assert!(state.tracking_abort.lock().expect("tracking abort mutex").is_none());
    assert!(state.send_abort.lock().expect("send abort mutex").is_none());
    assert!(state.recv_abort.lock().expect("recv abort mutex").is_none());
}

#[tokio::test]
async fn disconnect_is_idempotent() {
    let state = AppState::default();

    state.disconnect_ws();
    state.disconnect_ws();

    assert!(!state.is_ws_connected());
}
