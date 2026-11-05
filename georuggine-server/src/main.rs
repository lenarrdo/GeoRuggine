use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::mpsc;

use georuggine_server::{
    auth::user_repository::UserRepository,
    shared::{app_state::AppState, config::Config, database as db, logger},
    tui::{App, AppEvent},
    websocket::clients::ClientsList,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ------- process `.env` file
    let config = Config::init()
        .expect("Couldn't load '.env' file. Did you make a local copy of '.env.template'?");
    let host = config.host.clone();
    let port = config.port.clone();

    // ------- start logger
    logger::start(Duration::from_secs(120), &config.log_file)?;

    // ------- init router
    let (tx, rx) = mpsc::channel(100);
    let app_state = Arc::new(AppState::new(
        db::init(&config.db_url, false).await?,
        ClientsList::new(),
        tx.clone(),
        config,
    ));
    let router = georuggine_server::init_router(Arc::clone(&app_state));

    // ------- start server
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
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
    let log_msg = format!("Server listening on http://{host}:{port}",);
    let _ = tx.send(AppEvent::LogMessage(log_msg)).await;

    // ------- start TUI
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
