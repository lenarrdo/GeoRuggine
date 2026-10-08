pub mod api;
pub mod controllers;
pub(crate) mod domain;
pub(crate) mod repository;
pub(crate) mod services;

pub use api::dto::*;
#[allow(unused_imports)]
pub use controllers::tracking_controller::*;
pub use services::tracking_service::*;

pub async fn start_background_tracking(
    app_handle: &tauri::AppHandle,
    state: &crate::state::AppState,
    user_id: i64,
) -> Result<(), String> {
    TrackingService::start_background_tracking(app_handle, state, user_id).await
}

pub fn load_coordinates() -> Vec<(i64, i64)> {
    repository::coordinate_repository::CoordinateRepository::load_coordinates()
}
