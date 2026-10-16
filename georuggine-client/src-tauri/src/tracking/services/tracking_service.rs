use crate::state::AppState;
use crate::tracking::api::dto::PositionUpdate;
use crate::tracking::repository::coordinate_repository::CoordinateRepository;
use crate::websocket::api::dto::ClientIncomingMessage;
use std::time::Duration;
use tauri::Emitter;

pub struct TrackingService;

impl TrackingService {
    pub async fn start_background_tracking(
        app_handle: &tauri::AppHandle,
        state: &AppState,
    ) -> Result<(), String> {
        let coordinates = CoordinateRepository::load_coordinates();
        let app_clone = app_handle.clone();
        let state_clone = state.clone();

        let task_handle = tokio::spawn(async move {
            println!("[Tracking] Task 3 (Tracking periodico da file) avviato");
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            let mut index = 0;

            loop {
                interval.tick().await;

                let (x, y) = if index < coordinates.len() {
                    coordinates[index]
                } else {
                    *coordinates.last().unwrap_or(&(45000, 7600))
                };
                index += 1;

                let pos = PositionUpdate {
                    x_coord: x,
                    y_coord: y,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                // Invia la posizione sul canale condiviso MPSC verso il Task 1
                if let Err(e) = state_clone.send_ws_message(ClientIncomingMessage::Position(pos.clone())) {
                    eprintln!("[Tracking] Errore invio posizione sul canale MPSC: {e}");
                    break;
                }

                println!(
                    "[Tracking] Inviata posizione: x={x}, y={y}, time={}",
                    pos.timestamp
                );

                let _ = app_clone.emit("local-position-updated", pos);
            }
            println!("[Tracking] Task 3 (Tracking periodico) concluso");
        })
        .abort_handle();

        if let Ok(mut guard) = state.tracking_abort.lock() {
            if let Some(old) = guard.take() {
                old.abort();
            }
            *guard = Some(task_handle);
        }

        Ok(())
    }
}
