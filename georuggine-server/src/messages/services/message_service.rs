use crate::messages::api::dto::ChatMessage;
use crate::messages::repository::message_repository::MessageRepository;
use sqlx::SqlitePool;

#[allow(dead_code)]
pub struct MessageService;

#[allow(dead_code)]
impl MessageService {
    pub async fn save_incoming_chat(
        db_pool: &SqlitePool,
        msg: &ChatMessage,
    ) -> Result<(), sqlx::Error> {
        MessageRepository::save_message(db_pool, msg).await
    }
}
