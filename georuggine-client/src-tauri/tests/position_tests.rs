mod common;

use chrono::DateTime;
use georuggine_client_lib::messages::dto::ClientIncomingMessage;
use georuggine_client_lib::messages::position::PositionController;
use georuggine_client_lib::messages::position::tracking::start_background_tracking;
use georuggine_client_lib::state::AppState;
use std::time::Duration;

#[tokio::test]
async fn upload_is_rejected_before_parsing_when_disconnected() {
    let state = AppState::default();

    let error = PositionController::upload_coordinates(&state, "not-json")
        .await
        .expect_err("upload must fail when disconnected");

    assert!(error.contains("WebSocket connection to the server is not active"));
}

#[tokio::test]
async fn upload_rejects_invalid_json() {
    let (state, _rx) = common::connected_state();

    let error = PositionController::upload_coordinates(&state, "not-json")
        .await
        .expect_err("upload must fail");

    assert!(error.contains("does not contain a valid JSON coordinate format"));
}

#[tokio::test]
async fn upload_rejects_an_empty_coordinate_list() {
    let (state, _rx) = common::connected_state();

    let error = PositionController::upload_coordinates(&state, "[]")
        .await
        .expect_err("upload must fail");

    assert_eq!(error, "The JSON file does not contain any coordinate.");
}

#[tokio::test]
async fn upload_accepts_lat_lon_fields() {
    let (state, mut rx) = common::connected_state();

    PositionController::upload_coordinates(&state, r#"[{"lat":45.0703,"lon":7.6869}]"#)
        .await
        .expect("upload should start tracking");

    let message = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("tracking should enqueue the first point immediately")
        .expect("channel should be open");

    match message {
        ClientIncomingMessage::Position(position) => {
            assert_eq!(position.lat, 45.0703);
            assert_eq!(position.lon, 7.6869);
            let timestamp = position.timestamp.expect("tracking adds a timestamp");
            DateTime::parse_from_rfc3339(&timestamp).expect("timestamp must be RFC 3339");
        }
        ClientIncomingMessage::Text(_) => panic!("expected a position message"),
    }
}

#[tokio::test]
async fn upload_accepts_x_y_fields() {
    let (state, mut rx) = common::connected_state();

    PositionController::upload_coordinates(&state, r#"[{"x":1.5,"y":2.5}]"#)
        .await
        .expect("upload should start tracking");

    let message = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("tracking should enqueue the first point immediately")
        .expect("channel should be open");

    match message {
        ClientIncomingMessage::Position(position) => {
            assert_eq!(position.lat, 1.5);
            assert_eq!(position.lon, 2.5);
        }
        ClientIncomingMessage::Text(_) => panic!("expected a position message"),
    }
}

#[tokio::test]
async fn tracking_registers_an_abort_handle() {
    let (state, _rx) = common::connected_state();

    PositionController::upload_coordinates(&state, r#"[{"lat":1,"lon":2}]"#)
        .await
        .expect("upload should start tracking");

    assert!(state
        .tracking_abort
        .lock()
        .expect("tracking abort mutex")
        .is_some());

    state.disconnect_ws();
}

#[tokio::test]
async fn tracking_rejects_an_empty_coordinate_list() {
    let state = AppState::default();

    let error = start_background_tracking(&state, Vec::new())
        .await
        .expect_err("tracking must reject an empty list");

    assert_eq!(error, "No coordinates provided for tracking.");
}

#[tokio::test]
async fn a_new_tracking_replaces_the_previous_task() {
    let (state, _rx) = common::connected_state();

    PositionController::upload_coordinates(&state, r#"[{"lat":1,"lon":2}]"#)
        .await
        .expect("first upload");

    PositionController::upload_coordinates(&state, r#"[{"lat":3,"lon":4}]"#)
        .await
        .expect("second upload");

    assert!(state
        .tracking_abort
        .lock()
        .expect("tracking abort mutex")
        .is_some());

    state.disconnect_ws();
}
