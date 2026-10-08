pub mod api;
pub mod controllers;
pub(crate) mod domain;
pub(crate) mod repository;
pub mod services;

pub use api::dto::*;
pub use controllers::ws_controller::*;
pub use domain::connection::*;
pub use services::ws_service::*;

pub async fn connect_ws_internal(
    app_handle: &tauri::AppHandle,
    state: &crate::state::AppState,
    token: &str,
) -> Result<(), String> {
    WsService::connect_ws_internal(app_handle, state, token).await
}
