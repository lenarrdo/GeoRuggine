use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};

use crate::messages::TextMessage;

/// Rappresenta un client WebSocket attualmente connesso al server.
#[derive(Clone)]
pub struct ConnectedClient {
    pub tx: mpsc::UnboundedSender<TextMessage>,
}

impl ConnectedClient {
    pub fn new(tx: mpsc::UnboundedSender<TextMessage>) -> Self {
        ConnectedClient { tx }
    }
}

/// Registro condiviso di tutte le sessioni WebSocket attive.
pub struct ClientsList {
    list: RwLock<HashMap<u32, ConnectedClient>>,
}

impl ClientsList {
    pub fn new() -> Self {
        ClientsList {
            list: RwLock::new(HashMap::new()),
        }
    }

    /// Registra un nuovo client nella lista.
    /// Restituisce il numero di utenti connessi dopo l'aggiunta.
    pub async fn add(&self, user_id: u32, new_client: ConnectedClient) -> usize {
        let mut guard = self.list.write().await;
        guard.insert(user_id, new_client);
        guard.len()
    }

    /// Rimuove un client dalla lista.
    /// Restituisce il numero di utenti connessi dopo la rimozione.
    pub async fn remove(&self, user_id: u32) -> usize {
        let mut guard = self.list.write().await;
        guard.remove(&user_id);
        guard.len()
    }

    /// Restituisce il numero di client attualmente online.
    pub async fn len(&self) -> usize {
        self.list.read().await.len()
    }

    /// Restituisce true se un utente è connesso, false altrimenti.
    pub async fn is_connected(&self, user_id: u32) -> bool {
        self.list.read().await.contains_key(&user_id)
    }

    /// Invia un messaggio in BROADCAST a tutte le sessioni WebSocket attive.
    pub async fn send_all(&self, msg: &str) {
        let guard = self.list.read().await;

        for (user_id, client) in guard.iter() {
            let text_message = TextMessage::new(0, *user_id, msg.to_string());
            if let Err(e) = client.tx.send(text_message) {
                eprintln!("Errore inserimento messaggio in coda broadcast per {user_id}: {e}");
            }
        }
    }

    /// Invia un messaggio a un SINGOLO CLIENT, specificando l'user_id.
    pub async fn send_one(&self, target_user_id: u32, msg: &str) -> bool {
        let guard = self.list.read().await;

        let Some(client) = guard.get(&target_user_id) else {
            eprintln!("user_id {target_user_id} non trovato, messaggio non inviato.");
            return false;
        };

        let text_message = TextMessage::new(0, target_user_id, msg.to_string());
        if let Err(e) = client.tx.send(text_message) {
            eprintln!("Errore invio a user_id {target_user_id}: {e}");
            return false;
        }

        true
    }
}
