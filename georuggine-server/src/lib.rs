pub mod auth;
pub mod database;
pub mod messages;
pub mod websocket;

use crate::auth::{handle_login, handle_registration};
use crate::websocket::ws_handler;
use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use sqlx::SqlitePool;

pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "3000";
pub const BASE_URL: &str = "/api/v1";

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub clients: websocket::clients::ClientsList,
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for websocket::clients::ClientsList {
    fn from_ref(state: &AppState) -> Self {
        state.clients.clone()
    }
}

pub fn create_app(db_pool: SqlitePool, clients: websocket::clients::ClientsList) -> Router {
    let app_state = AppState {
        db: db_pool,
        clients,
    };

    Router::new()
        .route(
            &format!("{BASE_URL}/auth/register"),
            post(handle_registration),
        )
        .route(&format!("{BASE_URL}/auth/login"), post(handle_login))
        .route(&format!("{BASE_URL}/ws"), get(ws_handler))
        .with_state(app_state)
}
