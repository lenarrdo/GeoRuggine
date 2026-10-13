use crate::messages::ChatMessage;
<<<<<<< HEAD
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, RwLock};
=======
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc};

/// Messaggio restituito in caso di errore durante l'apertura di un socket.
#[derive(Debug, Clone, Serialize)]
pub struct WsResponse {
    pub message: String,
}
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84

/// Rappresenta un client WebSocket attualmente connesso al server.
#[derive(Clone)]
pub struct ConnectedClient {
<<<<<<< HEAD
    pub user_id: Option<i64>,
=======
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84
    pub tx: mpsc::UnboundedSender<ChatMessage>,
}

/// Registro condiviso di tutte le sessioni WebSocket attive.
<<<<<<< HEAD
pub type Clients = Arc<RwLock<HashMap<SocketAddr, ConnectedClient>>>;
=======
pub type Clients = Arc<RwLock<HashMap<i64, ConnectedClient>>>;
>>>>>>> 204584b9c90538055304bba7dbc776e049566d84
