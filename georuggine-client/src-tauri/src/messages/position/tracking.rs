use crate::messages::dto::ClientIncomingMessage;
use crate::messages::position::dto::PositionUpdate;
use crate::state::AppState;
use std::time::Duration;

pub async fn start_background_tracking(
    state: &AppState,
    coordinates: Vec<(f64, f64)>,
) -> Result<(), String> {
    if coordinates.is_empty() {
        return Err("No coordinates provided for tracking.".to_string());
    }

    let state_clone = state.clone();

    let task_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        let mut index = 0;

        loop {
            interval.tick().await;

            // File exhausted: every coordinate has been sent, the task ends
            let Some(&(x, y)) = coordinates.get(index) else {
                break;
            };

            let pos = PositionUpdate {
                lat: x,
                lon: y,
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            };

            // Send the position on the shared MPSC channel towards Task 1
            if state_clone
                .send_ws_message(ClientIncomingMessage::Position(pos))
                .is_err()
            {
                // Connection unavailable: close tasks and the MPSC channel
                state_clone.disconnect_ws();
                break;
            }

            index += 1;
        }
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
