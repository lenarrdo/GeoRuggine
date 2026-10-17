use serde::{Deserialize, Serialize};

/// Coordinate geografiche inviate dal client al server (il timestamp viene calcolato dal client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub x_coord: i64,
    pub y_coord: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Punto di coordinate letto da file (JSON o CSV)
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatePoint {
    #[serde(alias = "x_coord")]
    pub x: i64,
    #[serde(alias = "y_coord")]
    pub y: i64,
}
