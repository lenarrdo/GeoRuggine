use sqlx::SqlitePool;

use crate::messages::{PositionUpdate, TextMessage};

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
        position: &PositionUpdate,
        timestamp: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO positions (user_id, lat, lon, timestamp) VALUES (?, ?, ?, ?);",
        )
        .bind(user_id)
        .bind(position.lat)
        .bind(position.lon)
        .bind(timestamp)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    // FIXME
    // pub async fn get_messages_for_user(
    //     db_pool: &SqlitePool,
    //     user_id: u32,
    // ) -> Result<Vec<ChatMessage>, sqlx::Error> {
    //     let records = sqlx::query_as::<_, ChatMessage>(
    //         "SELECT message_id, sender_id, receiver_id, timestamp, contents FROM messages WHERE sender_id = ? OR receiver_id = ? ORDER BY timestamp ASC",
    //     )
    //     .bind(user_id)
    //     .bind(user_id)
    //     .fetch_all(db_pool)
    //     .await?;
    //
    //     Ok(records)
    // }
}
