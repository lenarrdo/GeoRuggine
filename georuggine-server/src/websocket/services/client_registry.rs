use crate::messages::ChatMessage;
use crate::websocket::api::dto::Clients;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

/// Funzione di utilità per inizializzare il registro dei client vuoto.
pub fn create_clients() -> Clients {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Invia un messaggio in BROADCAST a tutte le sessioni WebSocket attive.
#[allow(dead_code)]
pub async fn send_broadcast(clients: &Clients, msg: ChatMessage) {
    let guard = clients.read().await;
    for (addr, client) in guard.iter() {
        if let Err(e) = client.tx.send(msg.clone()) {
            eprintln!("Errore inserimento messaggio in coda broadcast per {addr}: {e}");
        }
    }
}

/// Invia un messaggio a un SINGOLO CLIENT specifico cercando il suo `user_id`.
#[allow(dead_code)]
pub async fn send_to_user(clients: &Clients, target_user_id: i64, msg: ChatMessage) -> bool {
    let guard = clients.read().await;
    let mut sent = false;
    for (addr, client) in guard.iter() {
        if client.user_id == Some(target_user_id) {
            if let Err(e) = client.tx.send(msg.clone()) {
                eprintln!("Errore invio a user {target_user_id} su {addr}: {e}");
            } else {
                sent = true;
            }
        }
    }
    sent
}

/// Invia un messaggio a un SINGOLO CLIENT identificato direttamente dal suo `SocketAddr`.
#[allow(dead_code)]
pub async fn send_to_addr(clients: &Clients, addr: &SocketAddr, msg: ChatMessage) -> bool {
    let guard = clients.read().await;
    if let Some(client) = guard.get(addr) {
        client.tx.send(msg).is_ok()
    } else {
        false
    }
}
