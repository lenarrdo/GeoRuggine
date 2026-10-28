use dashmap::DashMap;
use tokio::sync::mpsc::Sender;

use crate::messages::{Coordinates, TextMessage};
use crate::shared::utils::INACTIVE_THRESHOLD;
use crate::tui::chat_box::UIMessage;

#[derive(Clone)]
pub struct ConnectedClient {
    tx: Sender<TextMessage>,
    journey_id: Option<String>,
    positions: Vec<Coordinates>,
    messages: Vec<UIMessage>,
}

impl ConnectedClient {
    pub fn new(tx: Sender<TextMessage>) -> Self {
        ConnectedClient {
            tx,
            journey_id: None,
            positions: vec![],
            messages: vec![],
        }
    }
}

pub struct ClientsList {
    pub list: DashMap<u32, ConnectedClient>,
}

impl ClientsList {
    pub fn new() -> Self {
        ClientsList {
            list: DashMap::new(),
        }
    }

    pub async fn send_one(&self, user_id: u32, msg: &str) -> Result<(), String> {
        let Some(mut client) = self.list.get_mut(&user_id) else {
            return Err(format!(
                "user_id {user_id} non trovato, messaggio non inviato."
            ));
        };

        let text_message = TextMessage::new(msg.to_string());
        if let Err(e) = client.tx.send(text_message.clone()).await {
            return Err(format!("Errore invio a user_id {user_id}: {e}"));
        }

        client.messages.push(UIMessage::new(0, text_message));

        Ok(())
    }

    pub fn record_position(&self, user_id: u32, pos: Coordinates) {
        if let Some(mut client) = self.list.get_mut(&user_id) {
            client.positions.push(pos);
        }
    }

    pub fn is_moving(&self, user_id: u32) -> bool {
        let Some(client) = self.list.get(&user_id) else {
            return false;
        };

        // first position received, the user is moving.
        if client.positions.is_empty() {
            return true;
        }

        let last_position: &Coordinates = client.positions.last().unwrap();
        let first_same_as_last = client
            .positions
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
        self.list.get(&user_id)?.journey_id.clone()
    }

    pub fn set_journey_id(&self, user_id: u32, journey_id: &str) {
        if let Some(mut user) = self.list.get_mut(&user_id)
            && user.journey_id.is_none()
        {
            user.journey_id = Some(journey_id.to_string());
        }
    }

    pub fn save_client_message(&self, user_id: u32, text_message: TextMessage) {
        if let Some(mut client) = self.list.get_mut(&user_id) {
            client.messages.push(UIMessage::new(user_id, text_message));
        };
    }

    pub fn get_messages(&self, user_id: u32) -> Option<Vec<UIMessage>> {
        if let Some(client) = self.list.get(&user_id) {
            Some(client.messages.clone())
        } else {
            None
        }
    }
}
