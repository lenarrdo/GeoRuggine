pub mod auth;
pub mod messages;
pub mod shared;
pub mod tracking;
pub mod tui;
pub mod websocket;

use axum::{
    Router,
    routing::{get, post},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;

use crate::auth::{handle_login, handle_registration, user_repository::UserRepository};
use crate::shared::{app_state::AppState, database as db};
use crate::tui::App;
use crate::websocket::{clients::ClientsList, ws_handler};

pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "3000";
pub const BASE_URL: &str = "/api/v1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // channel to send messages to the UI
    let (tx, rx) = mpsc::channel(100);

    let db_pool = db::init(false).await?;
    let list = Arc::new(ClientsList::new());
    let users = UserRepository::get_users(&db_pool).await?;

    let router = Router::new()
        .route(
            &format!("{BASE_URL}/auth/register"),
            post(handle_registration),
        )
        .route(&format!("{BASE_URL}/auth/login"), post(handle_login))
        .route(&format!("{BASE_URL}/ws"), get(ws_handler))
        .with_state(AppState::new(db_pool, Arc::clone(&list), tx.clone()));

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();

    // println!("Server listening on http://{HOST}:{PORT}");

    tokio::spawn(async move {
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    let terminal = ratatui::init();
    let app_result = App::new(tx, rx, users, list).run(terminal).await;
    ratatui::restore();

    app_result
}

