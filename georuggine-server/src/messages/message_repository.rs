use sqlx::{QueryBuilder, SqlitePool};

use crate::{
    messages::{Coordinates, TextMessage},
    tui::TimeFrame,
};

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
        journey_id: &str,
        position: &Coordinates,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO positions (user_id, journey_id, lat, lon, timestamp) VALUES (?, ?, ?, ?, ?);",
        )
        .bind(user_id)
        .bind(journey_id)
        .bind(position.lat)
        .bind(position.lon)
        .bind(position.received_at().to_rfc3339())
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get_positions(
        &self,
        user_id: u32,
        time_frame: TimeFrame,
    ) -> Result<Vec<Coordinates>, sqlx::Error> {
        let mut query_builder =
            QueryBuilder::new("SELECT lat, lon, timestamp FROM positions WHERE user_id = ");
        query_builder.push_bind(user_id);

        let time_query = match time_frame {
            TimeFrame::Today => "AND date(timestamp) = date('now')",
            TimeFrame::ThisWeek => "AND strftime('%Y-%V', timestamp) = strftime('%Y-%V', 'now')",
            TimeFrame::ThisMonth => "AND strftime('%Y-%m', timestamp) = strftime('%Y-%m', 'now')",
        };
        query_builder.push(time_query);

        let positions = query_builder
            .build_query_as::<Coordinates>()
            .fetch_all(&self.db_pool)
            .await?;

        Ok(positions)
    }
}
