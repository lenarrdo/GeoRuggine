use std::fmt::Display;

use chrono::{DateTime, Local, Utc};

pub struct LogMessage {
    contents: String,
    timestamp: DateTime<Utc>,
}

impl LogMessage {
    pub fn new(contents: String) -> Self {
        LogMessage {
            contents,
            timestamp: Utc::now(),
        }
    }
}

impl Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.timestamp.with_timezone(&Local).format("%H:%M:%S"),
            self.contents
        )
    }
}
