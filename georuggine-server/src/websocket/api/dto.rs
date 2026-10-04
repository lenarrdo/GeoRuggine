use crate::messages::ChatMessage;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, RwLock};

/// Rappresenta un client WebSocket attualmente connesso al server.
#[derive(Clone)]
pub struct ConnectedClient {
    pub user_id: Option<i64>,
    pub tx: mpsc::UnboundedSender<ChatMessage>,
}

/// Registro condiviso di tutte le sessioni WebSocket attive.
pub type Clients = Arc<RwLock<HashMap<SocketAddr, ConnectedClient>>>;
