use std::{collections::VecDeque, fmt::Display};

use chrono::{DateTime, Local, Utc};
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, List, ListItem},
};

const MAX_STORED_MESSAGES: usize = 50;

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
        let box_height = block.inner(area).height as usize;

        let skip_count = self.messages.len().saturating_sub(box_height);
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .skip(skip_count)
            .map(|m| ListItem::new(Line::from(m.to_string())))
            .collect();

        let msg_list = List::new(messages).block(block);
        frame.render_widget(msg_list, area);
    }

    pub fn add_message(&mut self, msg: String) {
        self.messages.push_back(LogMessage::new(msg));
        if self.messages.len() > MAX_STORED_MESSAGES {
            self.messages.pop_front();
        }
    }
}
