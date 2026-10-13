<<<<<<< HEAD
use georuggine_server::{create_app, database as db, websocket, HOST, PORT};
=======
use georuggine_server::{HOST, PORT, create_app, database as db, websocket};
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84
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
