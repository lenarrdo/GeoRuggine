use axum::extract::FromRef;
use sqlx::SqlitePool;
use tokio::sync::mpsc::Sender;

use crate::{tui::AppEvent, websocket::clients::ClientsList};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub clients: ClientsList,
    pub tx: Sender<AppEvent>,
}

impl FromRef<AppState> for SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for ClientsList {
    fn from_ref(state: &AppState) -> Self {
        state.clients.clone()
    }
}

impl FromRef<AppState> for Sender<AppEvent> {
    fn from_ref(state: &AppState) -> Self {
        state.tx.clone()
    }
}

impl AppState {
    pub fn new(db_pool: SqlitePool, clients: ClientsList, tx: Sender<AppEvent>) -> Self {
        AppState {
            db: db_pool,
            clients,
            tx,
        }
    }
}
