use crate::tracking::api::dto::PositionUpdate;
use crate::tracking::repository::position_repository::PositionRepository;
use sqlx::SqlitePool;

pub struct TrackingService;

#[allow(dead_code)]
impl TrackingService {
    pub async fn record_position(
        db_pool: &SqlitePool,
<<<<<<< HEAD
        pos: &PositionUpdate,
    ) -> Result<(), sqlx::Error> {
        PositionRepository::save_position(db_pool, pos).await
=======
        user_id: i64,
        pos: &PositionUpdate,
    ) -> Result<(), sqlx::Error> {
        PositionRepository::save_position(db_pool, user_id, pos).await
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84
    }
}
