use crate::messages::ChatMessage;
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{RwLock, mpsc};

/// Messaggio restituito in caso di errore durante l'apertura di un socket.
#[derive(Debug, Clone, Serialize)]
pub struct WsResponse {
    pub message: String,
}

/// Rappresenta un client WebSocket attualmente connesso al server.
#[derive(Clone)]
pub struct ConnectedClient {
    pub user_id: Option<i64>,
    pub tx: mpsc::UnboundedSender<ChatMessage>,
}

/// Registro condiviso di tutte le sessioni WebSocket attive.
pub type Clients = Arc<RwLock<HashMap<SocketAddr, ConnectedClient>>>;
