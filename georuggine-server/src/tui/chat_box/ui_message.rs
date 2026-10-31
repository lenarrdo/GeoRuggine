use std::fmt::Display;

use chrono::{DateTime, Utc};

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
}
