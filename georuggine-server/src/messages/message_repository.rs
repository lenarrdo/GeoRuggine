use sqlx::SqlitePool;

use crate::messages::{Coordinates, TextMessage};

#[derive(Clone)]
pub struct MessageRepository {
    db_pool: SqlitePool,
}

impl MessageRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        MessageRepository { db_pool }
    }

    pub async fn save_text_message(
        &self,
        sender_id: u32,
        receiver_id: Option<u32>,
        msg: &TextMessage,
        timestamp: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO messages (sender_id, receiver_id, timestamp, contents) VALUES (?, ?, ?, ?);")
            .bind(sender_id)
            .bind(receiver_id)
            .bind(timestamp)
            .bind(&msg.contents)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn save_position(
        &self,
        user_id: u32,
        position: &Coordinates,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO positions (user_id, lat, lon, timestamp) VALUES (?, ?, ?, ?);")
            .bind(user_id)
            .bind(position.lat)
            .bind(position.lon)
            .bind(position.received_at().to_string())
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn get_positions(&self, user_id: u32) -> Result<(), sqlx::Error> {
        sqlx::query_as::<_, Coordinates>(
            "SELECT lat, lon, timestamp AS received_at FROM positions WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(())
    }
}
