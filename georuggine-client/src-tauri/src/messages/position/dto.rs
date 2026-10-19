use serde::{Deserialize, Serialize};

/// Coordinate geografiche inviate dal client al server (il timestamp viene calcolato dal client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub lat: i64,
    pub lon: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Punto di coordinate letto da file (JSON o CSV)
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatePoint {
    #[serde(alias = "lat")]
    pub x: i64,
    #[serde(alias = "lon")]
    pub y: i64,
}
