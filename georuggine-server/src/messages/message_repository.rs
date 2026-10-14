use sqlx::SqlitePool;

use crate::messages::TextMessage;

pub struct MessageRepository {
    db_pool: SqlitePool,
}

impl MessageRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        MessageRepository { db_pool }
    }

    pub async fn save_message(&self, msg: &TextMessage) -> Result<(), sqlx::Error> {
        let receiver = msg.receiver_id;
        sqlx::query("INSERT INTO messages (sender_id, receiver_id, timestamp, contents) VALUES (?, ?, ?, ?);")
            .bind(msg.sender_id)
            .bind(receiver)
            .bind(&msg.timestamp)
            .bind(&msg.contents)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    // FIXME
    // pub async fn get_messages_for_user(
    //     db_pool: &SqlitePool,
    //     user_id: i64,
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
