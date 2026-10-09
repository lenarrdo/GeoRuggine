use serde::{Deserialize, Serialize};

/// Coordinate geografiche/emulate inviate dal client al server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: i64,
    pub y_coord: i64,
    pub timestamp: String,
}
