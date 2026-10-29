use serde::{Deserialize, Serialize};

/// Geographic coordinates sent by the client to the server (the timestamp is computed by the client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: i64,
    pub y_coord: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Coordinate point read from a file (JSON or CSV)
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatePoint {
    #[serde(alias = "x_coord")]
    pub x: i64,
    #[serde(alias = "y_coord")]
    pub y: i64,
}
