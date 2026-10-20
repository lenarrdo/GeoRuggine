use crate::messages::dto::ClientIncomingMessage;
use crate::messages::position::dto::{CoordinatePoint, PositionUpdate};
use crate::state::AppState;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn find_coordinates_file() -> Option<PathBuf> {
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

pub fn load_coordinates() -> Vec<(f64, f64)> {
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
                        if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
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
        (45.01, 7.68),
        (45.01, 7.68),
        (45.02, 7.69),
        (45.02, 7.69),
        (45.03, 7.70),
    ]
}

pub async fn start_background_tracking(state: &AppState) -> Result<(), String> {
    let coordinates = load_coordinates();
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
                *coordinates.last().unwrap_or(&(45.00, 76.00))
            };
            index += 1;

            let pos = PositionUpdate {
                lat: x,
                lon: y,
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            };

            // Invia la posizione sul canale condiviso MPSC verso il Task 1
            if let Err(e) =
                state_clone.send_ws_message(ClientIncomingMessage::Position(pos.clone()))
            {
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
