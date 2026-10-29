use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WsParams {
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WsResponse {
    pub message: String,
}
