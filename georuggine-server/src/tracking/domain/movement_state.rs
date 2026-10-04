use serde::{Deserialize, Serialize};

/// Stato di movimento dell'utente conforme ai requisiti (fermo, in movimento, sconnesso)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum MovementState {
    Stopped,
    Moving,
    Disconnected,
}
