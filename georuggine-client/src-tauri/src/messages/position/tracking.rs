use crate::messages::dto::ClientIncomingMessage;
use crate::messages::position::dto::PositionUpdate;
use crate::state::AppState;
use std::time::Duration;

pub async fn start_background_tracking(
    state: &AppState,
    coordinates: Vec<(i64, i64)>,
) -> Result<(), String> {
    if coordinates.is_empty() {
        return Err("Nessuna coordinata fornita per il tracking.".to_string());
    }

    let state_clone = state.clone();

    let task_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        let mut index = 0;

        loop {
            interval.tick().await;

            // File esaurito: tutte le coordinate sono state inviate, il task termina
            let Some(&(x, y)) = coordinates.get(index) else {
                break;
            };

            let pos = PositionUpdate {
                x_coord: x,
                y_coord: y,
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            };

            // Invia la posizione sul canale condiviso MPSC verso il Task 1
            if state_clone
                .send_ws_message(ClientIncomingMessage::Position(pos))
                .is_err()
            {
               state_clone.disconnect_ws();
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
    