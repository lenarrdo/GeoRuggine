use sqlx::SqlitePool;
use tokio::sync::mpsc::{self, Sender};

use crate::{
    shared::{config::Config, database},
    tui::AppEvent,
    websocket::clients::ClientsList,
};

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

    pub async fn mock() -> Self {
        let config = Config::mock();
        let db = database::init(&config.db_url, true)
            .await
            .expect("failed to init database");
        let clients = ClientsList::new();
        let (tx, _rx) = mpsc::channel(100);

        AppState {
            config,
            tx,
            clients,
            db,
        }
    }
}
