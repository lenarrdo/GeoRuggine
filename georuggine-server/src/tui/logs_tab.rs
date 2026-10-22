use std::{collections::VecDeque, fmt::Display};

use chrono::{DateTime, Local, Utc};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, List, ListItem},
};

const MAX_MESSAGES: usize = 50;

struct LogMessage {
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

#[derive(Default)]
pub struct LogsTab {
    messages: VecDeque<LogMessage>,
}

impl LogsTab {
    pub fn render(&mut self, block: Block, frame: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .map(|m| {
                let content = Line::from(Span::raw(m.to_string()));
                ListItem::new(content)
            })
            .collect();
        let msg_list = List::new(messages).block(block);
        frame.render_widget(msg_list, area);
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push_back(LogMessage::new(msg));
        if self.messages.len() >= MAX_MESSAGES {
            self.messages.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
