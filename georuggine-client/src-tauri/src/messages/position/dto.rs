use serde::{Deserialize, Serialize};

/// Geographic coordinates sent by the client to the server (the timestamp is computed by the client)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub lat: f64,
    pub lon: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Coordinate point read from a file (JSON or CSV)
#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatePoint {
    #[serde(alias = "lat")]
    pub x: f64,
    #[serde(alias = "lon")]
    pub y: f64,
}
