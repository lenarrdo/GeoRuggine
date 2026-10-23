use crate::messages::position::dto::CoordinatePoint;
use crate::messages::position::tracking::start_background_tracking;
use crate::state::AppState;

pub struct PositionController;

impl PositionController {
    pub async fn upload_coordinates(state: &AppState, content: &str) -> Result<(), String> {
        // 1. Validazione del formato JSON e deserializzazione in CoordinatePoint
        let points: Vec<CoordinatePoint> = serde_json::from_str(content)
            .map_err(|e| format!("Il file non contiene un formato JSON valido di coordinate: {e}"))?;

        if points.is_empty() {
            return Err("Il file JSON non contiene alcuna coordinata.".to_string());
        }

        // 2. Mappatura in coordinate (x, y)
        let coordinates: Vec<(i64, i64)> = points.into_iter().map(|p| (p.x, p.y)).collect();

        println!(
            "[PositionController] Elaborate con successo {} coordinate dal file caricato",
            coordinates.len()
        );

        // 3. Avvio immediato del background tracking con le coordinate ricevute
        start_background_tracking(state, coordinates).await?;

        Ok(())
    }
}