use crate::messages::api::dto::ChatMessage;
use crate::messages::domain::message::Message;
use sqlx::SqlitePool;

#[allow(dead_code)]
pub struct MessageRepository;

#[allow(dead_code)]
impl MessageRepository {
    pub async fn save_message(db_pool: &SqlitePool, msg: &ChatMessage) -> Result<(), sqlx::Error> {
        let receiver = msg.receiver_id.unwrap_or(0);
        sqlx::query("INSERT INTO messages (sender_id, receiver_id, timestamp, contents) VALUES (?, ?, ?, ?);")
            .bind(msg.sender_id)
            .bind(receiver)
            .bind(&msg.timestamp)
            .bind(&msg.contents)
            .execute(db_pool)
            .await?;

        Ok(())
    }

    pub async fn get_messages_for_user(
        db_pool: &SqlitePool,
        user_id: i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        let records = sqlx::query_as::<_, Message>(
            "SELECT message_id, sender_id, receiver_id, timestamp, contents FROM messages WHERE sender_id = ? OR receiver_id = ? ORDER BY timestamp ASC",
        )
        .bind(user_id)
        .bind(user_id)
        .fetch_all(db_pool)
        .await?;

        Ok(records)
    }
}
