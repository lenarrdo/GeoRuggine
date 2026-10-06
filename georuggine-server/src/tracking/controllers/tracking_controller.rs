use crate::tracking::api::dto::PositionUpdate;
use crate::tracking::services::tracking_service::TrackingService;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::SqlitePool;

#[allow(dead_code)]
pub async fn handle_record_position(
    State(db_pool): State<SqlitePool>,
    Json(pos): Json<PositionUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    TrackingService::record_position(&db_pool, &pos)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
