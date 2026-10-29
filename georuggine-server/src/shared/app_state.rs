use sqlx::SqlitePool;
use tokio::sync::mpsc::Sender;

use crate::{shared::config::Config, tui::AppEvent, websocket::clients::ClientsList};

pub struct AppState {
    pub db: SqlitePool,
    pub clients: ClientsList,
    pub tx: Sender<AppEvent>,
    pub config: Config,
}

impl AppState {
    pub fn new(db: SqlitePool, clients: ClientsList, tx: Sender<AppEvent>, config: Config) -> Self {
        AppState {
            db,
            clients,
            tx,
            config,
        }
    }
}
