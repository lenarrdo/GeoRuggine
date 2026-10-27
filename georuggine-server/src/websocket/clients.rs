use std::collections::HashMap;
use std::sync::{RwLock, mpsc};

use crate::messages::{Coordinates, TextMessage};
use crate::shared::utils::INACTIVE_THRESHOLD;
use crate::tui::chat_box::UIMessage;

/// Rappresenta un client WebSocket attualmente connesso al server.
#[derive(Clone)]
pub struct ConnectedClient {
    tx: mpsc::Sender<TextMessage>,
    journey_id: Option<String>,
    positions: Vec<Coordinates>,
    messages: Vec<UIMessage>,
}

impl ConnectedClient {
    pub fn new(tx: mpsc::Sender<TextMessage>) -> Self {
        ConnectedClient {
            tx,
            journey_id: None,
            positions: vec![],
            messages: vec![],
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
    pub fn add(&self, user_id: u32, new_client: ConnectedClient) -> usize {
        let mut guard = self.list.write().unwrap();
        guard.insert(user_id, new_client);
        guard.len()
    }

    /// Rimuove un client dalla lista.
    /// Restituisce il numero di utenti connessi dopo la rimozione.
    pub fn remove(&self, user_id: u32) -> usize {
        let mut guard = self.list.write().unwrap();
        guard.remove(&user_id);
        guard.len()
    }

    /// Restituisce il numero di client attualmente online.
    pub fn len(&self) -> usize {
        self.list.read().unwrap().len()
    }

    /// Restituisce true se un utente è connesso, false altrimenti.
    pub fn is_connected(&self, user_id: u32) -> bool {
        self.list.read().unwrap().contains_key(&user_id)
    }

    /// Invia un messaggio a un SINGOLO CLIENT, specificando l'user_id.
    pub fn send_one(&self, user_id: u32, msg: &str) -> Result<(), String> {
        let mut guard = self.list.write().unwrap();

        let Some(client) = guard.get_mut(&user_id) else {
            return Err(format!(
                "user_id {user_id} non trovato, messaggio non inviato."
            ));
        };

        let text_message = TextMessage::new(msg.to_string());
        if let Err(e) = client.tx.send(text_message.clone()) {
            return Err(format!("Errore invio a user_id {user_id}: {e}"));
        }

        client.messages.push(UIMessage::new(0, text_message));

        Ok(())
    }

    /// Registra una nuova posizione per un dato utente
    pub fn record_position(&self, user_id: u32, pos: Coordinates) {
        let mut guard = self.list.write().unwrap();
        if let Some(client) = guard.get_mut(&user_id) {
            client.positions.push(pos);
        }
    }

    /// Restituisce la velocità media di un utente lungo un percorso
    /// oppure `None` se non ci sono dati a sufficienza per calcolare la velocità.
    pub fn get_avg_speed(&self, user_id: u32) -> Option<f64> {
        let guard = self.list.read().unwrap();
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

    /// Restituisce tutte le posizioni inviate da un utente a partire dall'apertura del WebSocket.
    fn get_positions(&self, user_id: u32) -> Option<Vec<Coordinates>> {
        let guard = self.list.read().unwrap();
        if let Some(client) = guard.get(&user_id) {
            return Some(client.positions.clone());
        }

        None
    }

    /// Restituisce true se l'utente è in movimento, false altrimenti.
    pub fn is_moving(&self, user_id: u32) -> bool {
        let Some(positions) = self.get_positions(user_id) else {
            return false;
        };

        // first position received, the user is moving.
        if positions.is_empty() {
            return true;
        }

        let last_position: &Coordinates = positions.last().unwrap();
        let first_same_as_last = positions
            .iter()
            .rev()
            .take_while(|&pos| pos == last_position)
            .last()
            .unwrap();

        if last_position
            .received_at()
            .signed_duration_since(first_same_as_last.received_at())
            > INACTIVE_THRESHOLD
        {
            return false;
        }

        true
    }

    pub fn get_journey_id(&self, user_id: u32) -> Option<String> {
        self.list.read().unwrap().get(&user_id)?.journey_id.clone()
    }

    pub fn set_journey_id(&self, user_id: u32, journey_id: &str) {
        let mut guard = self.list.write().unwrap();
        if let Some(user) = guard.get_mut(&user_id)
            && user.journey_id.is_none()
        {
            user.journey_id = Some(journey_id.to_string());
        }
    }

    pub fn save_client_message(&self, user_id: u32, text_message: TextMessage) {
        let mut guard = self.list.write().unwrap();
        let Some(client) = guard.get_mut(&user_id) else {
            return;
        };

        client.messages.push(UIMessage::new(user_id, text_message));
    }

    pub fn get_messages(&self, user_id: u32) -> Option<Vec<UIMessage>> {
        let guard = self.list.read().unwrap();
        let Some(client) = guard.get(&user_id) else {
            return None;
        };

        Some(client.messages.clone())
    }
}
