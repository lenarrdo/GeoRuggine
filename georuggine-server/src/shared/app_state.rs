use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::SqlitePool;
use tokio::sync::mpsc::Sender;

use crate::{tui::AppEvent, websocket::clients::ClientsList};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub clients: Arc<ClientsList>,
    pub tx: Sender<AppEvent>,
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Arc<ClientsList> {
    fn from_ref(state: &AppState) -> Self {
        Arc::clone(&state.clients)
    }
}

impl FromRef<AppState> for Sender<AppEvent> {
    fn from_ref(state: &AppState) -> Self {
        state.tx.clone()
    }
}

impl AppState {
    pub fn new(db_pool: SqlitePool, clients: Arc<ClientsList>, tx: Sender<AppEvent>) -> Self {
        AppState {
            db: db_pool,
            clients,
            tx,
        }
    }
}
