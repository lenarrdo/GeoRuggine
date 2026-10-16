use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CoordinatePoint {
    #[serde(alias = "x_coord")]
    pub x: i64,
    #[serde(alias = "y_coord")]
    pub y: i64,
}
