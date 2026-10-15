use axum::extract::FromRef;
use sqlx::SqlitePool;

use crate::websocket::clients::ClientsList;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub clients: ClientsList,
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

impl AppState {
    pub fn new(db_pool: SqlitePool, clients: ClientsList) -> Self {
        AppState {
            db: db_pool,
            clients,
        }
    }
}
