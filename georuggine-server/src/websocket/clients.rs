use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};

use crate::{messages::TextMessage, tracking::Coordinates};

/// Rappresenta un client WebSocket attualmente connesso al server.
#[derive(Clone)]
pub struct ConnectedClient {
    tx: mpsc::UnboundedSender<TextMessage>,
    positions: Vec<Coordinates>,
}

impl ConnectedClient {
    pub fn new(tx: mpsc::UnboundedSender<TextMessage>) -> Self {
        ConnectedClient {
            tx,
            positions: vec![],
        }
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

    /// Registra una nuova posizione per un dato utente
    pub async fn record_position(&self, user_id: u32, pos: Coordinates) {
        let mut guard = self.list.write().await;
        if let Some(client) = guard.get_mut(&user_id) {
            client.positions.push(pos);
        }
    }

    /// Restituisce la velocità media di un utente lungo un percorso
    /// oppure `None` se non ci sono dati a sufficienza per calcolare la velocità.
    pub async fn get_avg_speed(&self, user_id: u32) -> Option<f64> {
        let guard = self.list.read().await;
        let client = guard.get(&user_id)?;

        if client.positions.len() < 2 {
            return None;
        }

        let last_pos = client.positions.last().unwrap();
        let first_pos = client.positions.first().unwrap();

        let total_distance = last_pos.distance_from(first_pos);

        let duration_seconds = last_pos
            .received_at()
            .signed_duration_since(first_pos.received_at())
            .num_milliseconds() as f64
            / 1000.0;

        if duration_seconds <= 0.0 {
            return Some(0.0);
        }

        let total_hours = duration_seconds / 3600.0;

        Some(total_distance / total_hours)
    }

    pub async fn get_positions(&self, user_id: u32) -> Option<Vec<Coordinates>> {
        let guard = self.list.read().await;
        if let Some(client) = guard.get(&user_id) {
            return Some(client.positions.clone());
        }

        None
    }
}
