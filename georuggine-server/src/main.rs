mod database;

use crate::database as db;

use axum::{
    Router,
    routing::{delete, post},
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    db::init(false).await?;

    let app = Router::new()
        .route("/register", post(todo_route))
        .route("/login", post(todo_route))
        .route("/logout", delete(todo_route));

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn todo_route() -> &'static str {
    "TODO"
}
