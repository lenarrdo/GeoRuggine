use serde::{Deserialize, Serialize};

/// Coordinate geografiche inviate periodicamente al server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub user_id: i64,
    pub x_coord: i64,
    pub y_coord: i64,
    pub timestamp: String,
}
