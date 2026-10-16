use serde::{Deserialize, Serialize};

/// Coordinate geografiche inviate periodicamente al server (nessun timestamp né ID inviati dal client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: i64,
    pub y_coord: i64,
}
