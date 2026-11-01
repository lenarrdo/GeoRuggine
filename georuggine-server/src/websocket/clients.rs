use chrono::Utc;
use dashmap::DashMap;
use tokio::sync::mpsc::Sender;

use crate::{
    messages::{position::Coordinates, text::TextMessage},
    shared::utils::INACTIVE_THRESHOLD,
};

#[derive(Clone)]
pub struct ConnectedClient {
    tx: Sender<TextMessage>,
    journey_id: u32,
    last_position: Option<Coordinates>,
}

impl ConnectedClient {
    pub fn new(tx: Sender<TextMessage>, journey_id: u32) -> Self {
        ConnectedClient {
            tx,
            journey_id,
            last_position: None,
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
        let (tx, text_message) = {
            let client = self.list.get_mut(&user_id).ok_or(format!(
                "Couldn't send message: user #{user_id} not online."
            ))?;
            (client.tx.clone(), TextMessage::new(msg.to_string()))
        };

        tx.send(text_message.clone())
            .await
            .map_err(|e| format!("Couldn't send message to User #{user_id}: {e}"))?;

        Ok(())
    }

    pub fn update_position(&self, user_id: u32, new: Coordinates) {
        let Some(mut client) = self.list.get_mut(&user_id) else {
            return;
        };

        if client
            .last_position
            .as_ref()
            .is_none_or(|last| *last != new)
        {
            client.last_position = Some(new);
        }
    }

    pub fn is_moving(&self, user_id: u32) -> bool {
        let Some(client) = self.list.get(&user_id) else {
            return false;
        };

        let Some(last_position) = client.last_position.as_ref() else {
            return false;
        };

        Utc::now().signed_duration_since(last_position.timestamp) < INACTIVE_THRESHOLD
    }

    pub fn get_journey_id(&self, user_id: u32) -> Option<u32> {
        let Some(user) = self.list.get(&user_id) else {
            return None;
        };

        Some(user.journey_id)
    }
}
