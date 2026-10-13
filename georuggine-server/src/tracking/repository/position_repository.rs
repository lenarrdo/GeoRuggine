use crate::tracking::api::dto::PositionUpdate;
use crate::tracking::domain::position::Position;
use sqlx::SqlitePool;

pub struct PositionRepository;

#[allow(dead_code)]
impl PositionRepository {
    pub async fn save_position(
        db_pool: &SqlitePool,
<<<<<<< HEAD
        pos: &PositionUpdate,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO positions (user_id, x_coord, y_coord, timestamp) VALUES (?, ?, ?, ?);")
            .bind(pos.user_id)
            .bind(pos.x_coord)
            .bind(pos.y_coord)
            .bind(&pos.timestamp)
            .execute(db_pool)
            .await?;
=======
        user_id: i64,
        pos: &PositionUpdate,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO positions (user_id, x_coord, y_coord, timestamp) VALUES (?, ?, ?, ?);",
        )
        .bind(user_id)
        .bind(pos.x_coord)
        .bind(pos.y_coord)
        .bind(&pos.timestamp)
        .execute(db_pool)
        .await?;
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84

        Ok(())
    }

    pub async fn get_positions_by_user(
        db_pool: &SqlitePool,
        user_id: i64,
    ) -> Result<Vec<Position>, sqlx::Error> {
        let positions = sqlx::query_as::<_, Position>(
            "SELECT id, user_id, x_coord, y_coord, timestamp FROM positions WHERE user_id = ? ORDER BY timestamp DESC",
        )
        .bind(user_id)
        .fetch_all(db_pool)
        .await?;

        Ok(positions)
    }
}
