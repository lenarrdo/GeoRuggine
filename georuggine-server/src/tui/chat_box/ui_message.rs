use std::fmt::Display;

use chrono::{DateTime, Utc};

use crate::messages::text::TextMessage;

#[derive(Default, Clone)]
pub struct UIMessage {
    pub timestamp: DateTime<Utc>,
    pub contents: String,
    pub sender: u32,
}

impl Display for UIMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl UIMessage {
    pub fn new(sender: u32, contents: &str, timestamp: DateTime<Utc>) -> Self {
        UIMessage {
            sender,
            timestamp: timestamp,
            contents: contents.to_owned(),
        }
    }

    pub fn from(text_msg: &TextMessage, sender_id: u32) -> Self {
        UIMessage {
            timestamp: text_msg.timestamp,
            contents: text_msg.contents.to_owned(),
            sender: sender_id,
        }
    }
}
