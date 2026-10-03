use georuggine_server::{create_app, database as db, websocket, HOST, PORT};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = db::init(true).await?;
    let clients = websocket::create_clients();

    let app = create_app(db_pool, clients);

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();

    println!("Server listening on http://{HOST}:{PORT}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();

    Ok(())
}
