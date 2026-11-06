mod common;

use georuggine_client_lib::auth::models::dto::{AuthRequest, AuthResponse, AuthResult};
use georuggine_client_lib::auth::{AuthController, UserSession};
use georuggine_client_lib::state::AppState;

#[test]
fn current_user_is_none_without_a_session() {
    let state = AppState::default();

    assert_eq!(
        AuthController::get_current_user(&state).expect("current user"),
        None
    );
}

#[test]
fn current_user_returns_the_stored_username() {
    let state = AppState::default();
    state
        .save_session(UserSession {
            username: "alice".to_string(),
            token: "token".to_string(),
        })
        .expect("save session");

    assert_eq!(
        AuthController::get_current_user(&state).expect("current user"),
        Some("alice".to_string())
    );
}

#[test]
fn logout_clears_the_session() {
    let state = AppState::default();
    state
        .save_session(UserSession {
            username: "alice".to_string(),
            token: "token".to_string(),
        })
        .expect("save session");

    AuthController::logout(&state).expect("logout");

    assert!(state.get_session().expect("session").is_none());
    assert!(!state.is_ws_connected());
}

#[test]
fn logout_disconnects_an_active_websocket() {
    let (state, rx) = common::connected_state();
    state
        .save_session(UserSession {
            username: "alice".to_string(),
            token: "token".to_string(),
        })
        .expect("save session");

    assert!(state.is_ws_connected());

    AuthController::logout(&state).expect("logout");

    assert!(!state.is_ws_connected());
    assert!(state.get_session().expect("session").is_none());
    drop(rx);
}

#[test]
fn auth_request_serializes_the_expected_fields() {
    let request = AuthRequest {
        username: "alice".to_string(),
        password: "secret".to_string(),
    };

    let json = serde_json::to_value(&request).expect("serialize");

    assert_eq!(json["username"], "alice");
    assert_eq!(json["password"], "secret");
    assert_eq!(json.as_object().expect("object").len(), 2);
}

#[test]
fn auth_response_deserializes_a_token() {
    let response: AuthResponse =
        serde_json::from_str(r#"{"message":"ok","token":"jwt"}"#).expect("deserialize");

    assert_eq!(response.message, "ok");
    assert_eq!(response.token.as_deref(), Some("jwt"));
}

#[test]
fn auth_response_accepts_a_missing_token() {
    let response: AuthResponse =
        serde_json::from_str(r#"{"message":"registered"}"#).expect("deserialize");

    assert_eq!(response.message, "registered");
    assert!(response.token.is_none());
}

#[test]
fn auth_result_serializes_success_and_message_without_the_token() {
    let result = AuthResult {
        success: true,
        message: "welcome".to_string(),
    };

    let json = serde_json::to_value(&result).expect("serialize");

    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "welcome");
    assert!(json.get("token").is_none());
}
