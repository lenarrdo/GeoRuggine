pub mod log_message;

use std::collections::VecDeque;

use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, List, ListItem},
};

use crate::tui::logs_tab::log_message::LogMessage;

const MAX_STORED_MESSAGES: usize = 50;

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
