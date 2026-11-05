pub mod auth;
pub mod messages;
pub mod shared;
pub mod tui;
pub mod websocket;

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    auth::{handle_login, handle_registration},
    shared::app_state::AppState,
    websocket::ws_handler,
};

pub const BASE_URL: &str = "/api/v1";

pub fn init_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            &format!("{BASE_URL}/auth/register"),
            post(handle_registration),
        )
        .route(&format!("{BASE_URL}/auth/login"), post(handle_login))
        .route(&format!("{BASE_URL}/ws"), get(ws_handler))
        .with_state(app_state)
}
