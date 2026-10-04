use crate::models::{ClientIncomingMessage, PositionUpdate};
use crate::state::AppState;
use chrono::Utc;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tauri::Emitter;

#[derive(Debug, Clone, Deserialize)]
struct CoordinatePoint {
    #[serde(alias = "x_coord")]
    x: i64,
    #[serde(alias = "y_coord")]
    y: i64,
}

fn find_coordinates_file() -> Option<PathBuf> {
    let candidate_paths = [
        "coordinates.json",
        "positions.json",
        "../coordinates.json",
        "../positions.json",
        "coordinates.csv",
        "../coordinates.csv",
    ];

    for path in &candidate_paths {
        let p = Path::new(path);
        if p.exists() {
            println!("[Tracking] Trovato file coordinate: {}", p.display());
            return Some(p.to_path_buf());
        }
    }
    None
}

pub fn load_coordinates() -> Vec<(i64, i64)> {
    if let Some(path) = find_coordinates_file() {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(points) = serde_json::from_reader::<_, Vec<CoordinatePoint>>(reader) {
                    if !points.is_empty() {
                        println!("[Tracking] Caricate {} coordinate da JSON", points.len());
                        return points.into_iter().map(|p| (p.x, p.y)).collect();
                    }
                }
            } else {
                let mut points = Vec::new();
                for line in reader.lines().flatten() {
                    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                    if parts.len() >= 2 {
                        if let (Ok(x), Ok(y)) = (parts[0].parse::<i64>(), parts[1].parse::<i64>()) {
                            points.push((x, y));
                        }
                    }
                }
                if !points.is_empty() {
                    println!("[Tracking] Caricate {} coordinate da CSV", points.len());
                    return points;
                }
            }
        }
    }

    println!("[Tracking] File non trovato o non leggibile, impiego percorso predefinito");
    vec![
        (45010, 7680),
        (45015, 7685),
        (45020, 7690),
        (45025, 7695),
        (45030, 7700),
    ]
}

// ─────────────────────────────────────────────────────────────
// PRODUTTORE 2 (Task 3): TRACKING (Periodico ogni 30s da file)
// Legge in modo autonomo e sincronizzato dal file e deposita
// la coordinata nel medesimo canale MPSC verso il Task 1
// ─────────────────────────────────────────────────────────────
pub async fn start_background_tracking(
    app_handle: &tauri::AppHandle,
    state: &AppState,
    user_id: i64,
) -> Result<(), String> {
    state.stop_tracking();

    let coordinates = load_coordinates();
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
                user_id,
                x_coord: x,
                y_coord: y,
                timestamp: Utc::now().to_rfc3339(),
            };

            // Invia la posizione sul canale condiviso MPSC verso il Task 1
            if let Err(e) = state_clone.send_ws_message(ClientIncomingMessage::Position(pos.clone())) {
                eprintln!("[Tracking] Errore invio posizione sul canale MPSC: {e}");
                break;
            }

            println!(
                "[Tracking] Inviata posizione (User {user_id}): x={x}, y={y}, time={}",
                pos.timestamp
            );

            let _ = app_clone.emit("local-position-updated", pos);
        }
        println!("[Tracking] Task 3 (Tracking periodico) concluso");
    })
    .abort_handle();

    if let Ok(mut guard) = state.tracking_abort.lock() {
        *guard = Some(task_handle);
    }

    Ok(())
}
