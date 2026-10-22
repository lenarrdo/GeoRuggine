use axum::{
    Router,
    routing::{get, post},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;

use georuggine_server::shared::{app_state::AppState, database as db};
use georuggine_server::tui::App;
use georuggine_server::websocket::{clients::ClientsList, ws_handler};
use georuggine_server::{
    auth::{handle_login, handle_registration, user_repository::UserRepository},
    tui::AppEvent,
};

pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "3000";
pub const BASE_URL: &str = "/api/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // channel to send messages to the UI
    let (tx, rx) = mpsc::channel(100);

    let app_state = Arc::new(AppState::new(
        db::init(false).await?,
        ClientsList::new(),
        tx.clone(),
    ));
    let router = Router::new()
        .route(
            &format!("{BASE_URL}/auth/register"),
            post(handle_registration),
        )
        .route(&format!("{BASE_URL}/auth/login"), post(handle_login))
        .route(&format!("{BASE_URL}/ws"), get(ws_handler))
        .with_state(Arc::clone(&app_state));

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();

    tx.send(AppEvent::LogMessage(format!(
        "Server listening on http://{HOST}:{PORT}"
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

    let users = UserRepository::get_users(&app_state.db).await?;
    let terminal = ratatui::init();
    let app_result = App::new(rx, users, Arc::clone(&app_state))
        .run(terminal)
        .await;
    ratatui::restore();

    app_result
}
