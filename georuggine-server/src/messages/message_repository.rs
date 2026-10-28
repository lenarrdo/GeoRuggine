use chrono::{DateTime, Utc};
use sqlx::{QueryBuilder, SqlitePool};
use std::collections::HashMap;

use crate::{messages::Coordinates, tui::TimeFrame};

#[derive(sqlx::FromRow)]
struct PositionRow {
    journey_id: String,
    lat: f64,
    lon: f64,
    timestamp: DateTime<Utc>,
}

#[derive(Clone)]
pub struct MessageRepository {
    db_pool: SqlitePool,
}

impl MessageRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        MessageRepository { db_pool }
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
    ) -> Result<Vec<Vec<Coordinates>>, sqlx::Error> {
        let mut query_builder = QueryBuilder::new(
            "SELECT lat, lon, journey_id, timestamp FROM positions WHERE user_id =",
        );
        query_builder.push_bind(user_id);

        let time_query = match time_frame {
            TimeFrame::Today => "AND date(timestamp) = date('now')",
            TimeFrame::ThisWeek => "AND strftime('%Y-%V', timestamp) = strftime('%Y-%V', 'now')",
            TimeFrame::ThisMonth => "AND strftime('%Y-%m', timestamp) = strftime('%Y-%m', 'now')",
        };
        query_builder.push(time_query);

        query_builder.push("ORDER BY timestamp ASC;");

        let rows = query_builder
            .build_query_as::<PositionRow>()
            .fetch_all(&self.db_pool)
            .await?;

        let mut map: HashMap<String, Vec<Coordinates>> = HashMap::new();

        for row in rows {
            map.entry(row.journey_id).or_default().push(Coordinates {
                lat: row.lat,
                lon: row.lon,
                timestamp: row.timestamp,
            });
        }

        let positions: Vec<Vec<Coordinates>> = map.into_values().collect();

        Ok(positions)
    }
}
