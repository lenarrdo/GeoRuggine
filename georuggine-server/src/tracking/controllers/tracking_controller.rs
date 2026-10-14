use crate::{messages::PositionUpdate, tracking::services::tracking_service::TrackingService};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;

#[allow(dead_code)]
pub async fn handle_record_position(
    State(db_pool): State<SqlitePool>,
    user_id: i64,
    Json(pos): Json<PositionUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    TrackingService::record_position(&db_pool, user_id, &pos)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
