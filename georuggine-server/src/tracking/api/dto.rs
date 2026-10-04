use serde::{Deserialize, Serialize};

/// Coordinate geografiche/emulate inviate dal client al server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub user_id: i64,
    pub x_coord: i64,
    pub y_coord: i64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MovementStatusDto {
    pub user_id: i64,
    pub state: String,
    pub last_seen: String,
}
