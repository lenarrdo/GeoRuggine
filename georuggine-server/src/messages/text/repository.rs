use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::messages::text::TextMessage;

#[derive(sqlx::FromRow)]
pub struct TextMessageRow {
    pub id: u32,
    pub sender_id: u32,
    pub receiver_id: u32,
    pub journey_id: u32,
    pub timestamp: DateTime<Utc>,
    pub contents: String,
}

#[derive(Clone)]
pub struct TextMessageRepository {
    db_pool: SqlitePool,
}

impl TextMessageRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        TextMessageRepository { db_pool }
    }

    pub async fn save_message(
        &self,
        sender_id: u32,
        receiver_id: u32,
        journey_id: u32,
        text_message: &TextMessage,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO text_messages (sender_id, receiver_id, journey_id, timestamp, contents) \
             VALUES (?, ?, ?, ?, ?);",
        )
        .bind(sender_id)
        .bind(receiver_id)
        .bind(journey_id)
        .bind(&text_message.timestamp)
        .bind(&text_message.contents)
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    /// returns all the messages sent in a given journey_id
    pub async fn get_messages(&self, journey_id: u32) -> Result<Vec<TextMessageRow>, sqlx::Error> {
        let messages = sqlx::query_as::<_, TextMessageRow>(
            "SELECT id, sender_id, receiver_id, journey_id, timestamp, contents \
             FROM text_messages \
             WHERE journey_id = ? \
             ORDER BY timestamp;",
        )
        .bind(journey_id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(messages)
    }
}
