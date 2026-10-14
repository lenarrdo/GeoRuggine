use georuggine_server::{
    HOST, PORT, create_app, database as db, tui::App, websocket::clients::ClientsList,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db::init(true).await?;
    let clients = ClientsList::new();

    let app = create_app(db_pool, clients);

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();

    println!("Server listening on http://{HOST}:{PORT}");

    tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    ratatui::run(|terminal| App::new().run(terminal))
}
