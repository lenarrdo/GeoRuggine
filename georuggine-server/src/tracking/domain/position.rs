use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Position {
    pub id: i64,
    pub user_id: i64,
    pub x_coord: i64,
    pub y_coord: i64,
    pub timestamp: String,
}
