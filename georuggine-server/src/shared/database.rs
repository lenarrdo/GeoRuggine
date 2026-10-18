use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{fs, path::Path, str::FromStr};

const DATA_DIR: &str = "data";
const DB_NAME: &str = "georust.sqlite";

pub async fn init(reset_tables: bool) -> Result<Pool<Sqlite>, Box<dyn std::error::Error>> {
    fs::create_dir_all(DATA_DIR)?;
    let db_path = Path::new(DATA_DIR).join(DB_NAME);
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path.display()))?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;

    let mut tx = pool.begin().await?;

    if reset_tables {
        sqlx::query("DROP TABLE IF EXISTS messages")
            .execute(&mut *tx)
            .await?;
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
            x_coord INTEGER NOT NULL,
            y_coord INTEGER NOT NULL,
            timestamp DATETIME NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );",
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            message_id INTEGER PRIMARY KEY,
            sender_id INTEGER NOT NULL,
            receiver_id INTEGER,
            timestamp DATETIME NOT NULL,
            contents TEXT,
            FOREIGN KEY(sender_id) REFERENCES users(id),
            FOREIGN KEY(receiver_id) REFERENCES users(id)
        );",
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(pool)
}
