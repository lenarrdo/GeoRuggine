use crate::tracking::api::dto::PositionUpdate;
use crate::tracking::repository::position_repository::PositionRepository;
use sqlx::SqlitePool;

pub struct TrackingService;

#[allow(dead_code)]
impl TrackingService {
    pub async fn record_position(
        db_pool: &SqlitePool,
        user_id: i64,
        pos: &PositionUpdate,
    ) -> Result<(), sqlx::Error> {
        PositionRepository::save_position(db_pool, user_id, pos).await
    }
}
