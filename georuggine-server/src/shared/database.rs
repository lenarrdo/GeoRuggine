use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::str::FromStr;

use crate::shared::utils;

pub async fn init(
    db_path: &str,
    reset_tables: bool,
) -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    utils::ensure_dir_exists(db_path)?;
    let options = SqliteConnectOptions::from_str(db_path)?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;

    let mut tx = pool.begin().await?;

    if reset_tables {
        sqlx::query("DROP TABLE IF EXISTS users")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS journeys")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS positions")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS text_messages")
            .execute(&mut *tx)
            .await?;
    }

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             username TEXT UNIQUE NOT NULL,
             password_hash TEXT NOT NULL
         );",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS journeys (
            id INTEGER PRIMARY KEY,
            started_by INTEGER NOT NULL,
            started_at DATETIME NOT NULL,
            FOREIGN KEY(started_by) REFERENCES users(id)
        );",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS text_messages (
            id INTEGER PRIMARY KEY,
            sender_id INTEGER NOT NULL,
            receiver_id INTEGER NOT NULL,
            journey_id INTEGER NOT NULL,
            timestamp DATETIME NOT NULL,
            contents TEXT,
            FOREIGN KEY(sender_id) REFERENCES users(id),
            FOREIGN KEY(receiver_id) REFERENCES users(id),
            FOREIGN KEY(journey_id) REFERENCES journeys(id)
        );",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS positions (
            id INTEGER PRIMARY KEY,
            journey_id INTEGER NOT NULL,
            lat INTEGER NOT NULL,
            lon INTEGER NOT NULL,
            timestamp DATETIME NOT NULL,
            FOREIGN KEY(journey_id) REFERENCES journeys(id)
        );",
    )
    .execute(&mut *tx)
    .await?;

    if reset_tables {
        // need to recreate special "server" user if table were reset
        sqlx::query(
            "INSERT INTO users (id, username, password_hash) \
             VALUES (0, 'system@local', '');",
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(pool)
}
