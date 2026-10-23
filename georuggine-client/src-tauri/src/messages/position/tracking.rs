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
        println!(
            "[Tracking] Task 3 (Tracking periodico) avviato con {} coordinate",
            coordinates.len()
        );
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
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            };

            // Invia la posizione sul canale condiviso MPSC verso il Task 1
            if let Err(e) = state_clone.send_ws_message(ClientIncomingMessage::Position(pos.clone())) {
                eprintln!("[Tracking] Errore invio posizione sul canale MPSC: {e}");
                break;
            }

            println!(
                "[Tracking] Inviata posizione: x={x}, y={y}, time={:?}",
                pos.timestamp
            );
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
    