use crate::messages::position::dto::CoordinatePoint;
use crate::messages::position::tracking::start_background_tracking;
use crate::state::AppState;

pub struct PositionController;

impl PositionController {
    pub async fn upload_coordinates(state: &AppState, content: &str) -> Result<(), String> {
        // 0. Upfront check that the WebSocket connection is active
        if !state.is_ws_connected() {
            return Err("Unable to start tracking: the WebSocket connection to the server is not active. Sign in or check your network status.".to_string());
        }

        // 1. JSON format validation and deserialization into CoordinatePoint
        let points: Vec<CoordinatePoint> = serde_json::from_str(content)
            .map_err(|e| format!("The file does not contain a valid JSON coordinate format: {e}"))?;

        if points.is_empty() {
            return Err("The JSON file does not contain any coordinate.".to_string());
        }

        // 2. Mapping into (x, y) coordinates
        let coordinates: Vec<(i64, i64)> = points.into_iter().map(|p| (p.x, p.y)).collect();

        // 3. Immediate start of the background tracking with the received coordinates
        start_background_tracking(state, coordinates).await?;

        Ok(())
    }
}
