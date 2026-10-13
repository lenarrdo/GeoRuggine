use crate::messages::ChatMessage;
use crate::websocket::api::dto::Clients;
use std::{collections::HashMap, sync::Arc};
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

/// Invia un messaggio a un SINGOLO CLIENT, specificando l'user_id.
#[allow(dead_code)]
pub async fn send_to_user(clients: &Clients, target_user_id: i64, msg: ChatMessage) -> bool {
    let guard = clients.read().await;

    let Some(client) = guard.get(&target_user_id) else {
        eprintln!("user_id {target_user_id} non trovato, messaggio non inviato.");
        return false;
    };

    if let Err(e) = client.tx.send(msg) {
        eprintln!("Errore invio a user_id {target_user_id}: {e}");
        return false;
    }

    true
}

/// Invia un messaggio a un SINGOLO CLIENT identificato dal suo user_id.
#[allow(dead_code)]
pub async fn send_to_addr(clients: &Clients, user_id: i64, msg: ChatMessage) -> bool {
    clients
        .read()
        .await
        .get(&user_id)
        .map_or(false, |client| client.tx.send(msg).is_ok())
}
