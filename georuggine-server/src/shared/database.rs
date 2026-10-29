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
        sqlx::query("DROP TABLE IF EXISTS positions")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DROP TABLE IF EXISTS users")
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
        "CREATE TABLE IF NOT EXISTS positions (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            journey_id TEXT NOT NULL,
            lat REAL NOT NULL,
            lon REAL NOT NULL,
            timestamp DATETIME NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );",
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(pool)
}
