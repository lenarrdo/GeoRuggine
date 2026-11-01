use chrono::{DateTime, Utc};
use sqlx::{QueryBuilder, SqlitePool};
use std::collections::HashMap;

use crate::{
    messages::{Coordinates, position::Journey},
    tui::TimeFrame,
};

#[derive(sqlx::FromRow)]
struct PositionRow {
    journey_id: u32,
    lat: f64,
    lon: f64,
    timestamp: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct JourneyRow {
    pub id: u32,
    pub started_by: u32,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct PositionRepository {
    db_pool: SqlitePool,
}

impl PositionRepository {
    pub fn new(db_pool: SqlitePool) -> Self {
        PositionRepository { db_pool }
    }

    pub async fn save_position(
        &self,
        journey_id: u32,
        position: &Coordinates,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO positions (journey_id, lat, lon, timestamp) VALUES (?, ?, ?, ?);")
            .bind(journey_id)
            .bind(position.lat)
            .bind(position.lon)
            .bind(position.timestamp.to_rfc3339())
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    pub async fn get_positions(
        &self,
        user_id: u32,
        time_frame: TimeFrame,
    ) -> Result<Vec<Journey>, sqlx::Error> {
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

        let mut positions: HashMap<u32, Vec<Coordinates>> = HashMap::new();
        for row in rows {
            positions
                .entry(row.journey_id)
                .or_default()
                .push(Coordinates {
                    lat: row.lat,
                    lon: row.lon,
                    timestamp: row.timestamp,
                });
        }

        let journeys: Vec<Journey> = positions
            .into_iter()
            .map(|(key, coords)| Journey::new(key, coords))
            .collect();

        Ok(journeys)
    }

    /// creates a new journey for a given user.
    /// returns the new journey if successful
    pub async fn create_new_journey(&self, user_id: u32) -> Result<u32, sqlx::Error> {
        let id = sqlx::query(
            "INSERT INTO journeys (started_by, started_at) \
             VALUES (?, ?) \
             RETURNING id;
             ",
        )
        .bind(user_id)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.db_pool)
        .await?
        .last_insert_rowid();

        Ok(id as u32)
    }
}
