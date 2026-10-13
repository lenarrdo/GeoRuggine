use crate::tracking::api::dto::PositionUpdate;
use crate::tracking::services::tracking_service::TrackingService;
<<<<<<< HEAD
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
=======
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84
use sqlx::SqlitePool;

#[allow(dead_code)]
pub async fn handle_record_position(
    State(db_pool): State<SqlitePool>,
<<<<<<< HEAD
    Json(pos): Json<PositionUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    TrackingService::record_position(&db_pool, &pos)
=======
    user_id: i64,
    Json(pos): Json<PositionUpdate>,
) -> Result<impl IntoResponse, StatusCode> {
    TrackingService::record_position(&db_pool, user_id, &pos)
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
