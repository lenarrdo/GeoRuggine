pub mod ui_message;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{shared::utils::SERVER_ID, tui::chat_box::ui_message::UIMessage};

#[derive(Default)]
pub struct ChatBox {
    pub messages: Vec<UIMessage>,
    scroll_offset: usize,
}

impl ChatBox {
    pub fn render(&mut self, block: Block, frame: &mut Frame, area: Rect) {
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        if self.messages.is_empty() || inner_area.height == 0 {
            return;
        }

        const BUBBLE_HEIGHT: u16 = 3;
        let max_visible = (inner_area.height / BUBBLE_HEIGHT) as usize;
        let total_messages = self.messages.len();

        if self.scroll_offset > total_messages.saturating_sub(max_visible) {
            self.scroll_offset = total_messages.saturating_sub(max_visible);
        }

        let visible_messages: Vec<&UIMessage> = self
            .messages
            .iter()
            .skip(self.scroll_offset)
            .take(max_visible)
            .collect();

        let message_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                visible_messages
                    .iter()
                    .map(|_| Constraint::Length(BUBBLE_HEIGHT))
                    .collect::<Vec<Constraint>>(),
            )
            .split(inner_area);

        for (i, msg) in visible_messages.iter().enumerate() {
            if i >= message_chunks.len() {
                break;
            }

            let chunk = message_chunks[i];
            let text = msg.to_string();
            let text_width = (text.chars().count() as u16 + 4).min(chunk.width);

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(if msg.sender == SERVER_ID {
                    [Constraint::Min(0), Constraint::Length(text_width)]
                } else {
                    [Constraint::Length(text_width), Constraint::Min(0)]
                })
                .split(chunk);

            let formatted_timestamp = msg.timestamp.format("%H:%M").to_string();

            let bubble_block = Block::default()
                .borders(Borders::ALL)
                .style(if msg.sender == SERVER_ID {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::Gray)
                })
                .title(if msg.sender == SERVER_ID {
                    Line::from(formatted_timestamp).right_aligned()
                } else {
                    Line::from(formatted_timestamp).left_aligned()
                });

            let paragraph = Paragraph::new(text).block(bubble_block);

            let position = if msg.sender == SERVER_ID { 1 } else { 0 };
            frame.render_widget(paragraph, horizontal_layout[position]);
        }

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(total_messages / BUBBLE_HEIGHT as usize)
            .position(self.scroll_offset)
            .viewport_content_length(max_visible);

        let scrollbar = Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight);

        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_offset + 1 < self.messages.len() {
            self.scroll_offset += 1;
        }
    }
}
