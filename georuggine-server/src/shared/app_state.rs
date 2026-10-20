use sqlx::SqlitePool;
use tokio::sync::mpsc::Sender;

use crate::{tui::AppEvent, websocket::clients::ClientsList};

pub struct AppState {
    pub db: SqlitePool,
    pub clients: ClientsList,
    pub tx: Sender<AppEvent>,
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
