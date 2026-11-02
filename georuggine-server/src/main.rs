use axum::{
    Router,
    routing::{get, post},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;

use georuggine_server::{
    auth::{handle_login, handle_registration, user_repository::UserRepository},
    shared::{app_state::AppState, config::Config, database as db, logger, utils},
    tui::{App, AppEvent},
    websocket::{clients::ClientsList, ws_handler},
};

pub const BASE_URL: &str = "/api/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::init()
        .expect("Couldn't load '.env' file. Did you make a local copy of '.env.template'?");

    utils::ensure_dir_exists(&config.log_file)?;
    logger::start(std::time::Duration::from_secs(120), &config.log_file);

    // channel to send messages to the UI
    let (tx, rx) = mpsc::channel(100);

    let (host, port) = (config.host.clone(), config.port.clone());
    let app_state = Arc::new(AppState::new(
        db::init(&config.db_url, false).await?,
        ClientsList::new(),
        tx.clone(),
        config,
    ));
    let router = Router::new()
        .route(
            &format!("{BASE_URL}/auth/register"),
            post(handle_registration),
        )
        .route(&format!("{BASE_URL}/auth/login"), post(handle_login))
        .route(&format!("{BASE_URL}/ws"), get(ws_handler))
        .with_state(Arc::clone(&app_state));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    tx.send(AppEvent::LogMessage(format!(
        "Server listening on http://{}:{}",
        host, port
    )))
    .await
    .unwrap();

    tokio::spawn(async move {
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    let users = UserRepository::new(app_state.db.clone())
        .get_users()
        .await?
        .into_iter()
        .filter(|user| user.id != 0)
        .collect();
    let terminal = ratatui::init();
    let app_result = App::new(rx, users, Arc::clone(&app_state))
        .run(terminal)
        .await;
    ratatui::restore();

    app_result
}
