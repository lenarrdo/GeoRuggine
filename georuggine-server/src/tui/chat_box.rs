use chrono::{DateTime, Utc};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use std::fmt::Display;

use crate::messages::TextMessage;

#[derive(Default)]
pub struct UIMessage {
    timestamp: DateTime<Utc>,
    contents: String,
    sender: u32,
}

impl Display for UIMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl UIMessage {
    fn new(sender: u32, msg: TextMessage) -> Self {
        let date_time = msg.timestamp.parse().unwrap_or(Utc::now());
        UIMessage {
            sender,
            timestamp: date_time,
            contents: msg.contents,
        }
    }
}

#[derive(Default)]
pub struct ChatBox {
    messages: Vec<UIMessage>,
}

impl ChatBox {
    pub fn sample() -> Self {
        ChatBox {
            messages: vec![
                UIMessage::new(
                    0,
                    TextMessage {
                        contents: "Hi from server.".to_string(),
                        timestamp: Utc::now().to_string(),
                    },
                ),
                UIMessage::new(
                    1,
                    TextMessage {
                        contents: "Hi from client. This is a longer messageeeeeeeeeeeeeeeeeee."
                            .to_string(),
                        timestamp: Utc::now().to_string(),
                    },
                ),
                UIMessage::new(
                    0,
                    TextMessage {
                        contents: "Yeeeeeeeeeeeeeeeeeeeeeeeeeeeeeah (from server.)".to_string(),
                        timestamp: Utc::now().to_string(),
                    },
                ),
            ],
        }
    }

    pub fn render(&mut self, block: Block, frame: &mut Frame, area: Rect) {
        let message_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.messages
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(block.inner(area));

        for (i, msg) in self.messages.iter().enumerate() {
            if i >= message_chunks.len() {
                break;
            }

            let chunk = message_chunks[i];
            let text = msg.to_string();
            let text_width = (text.chars().count() as u16 + 4).min(chunk.width);

            let horizontal_layout = if msg.sender == 0 {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Length(text_width), Constraint::Min(0)])
                    .split(chunk)
            } else {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Min(0), Constraint::Length(text_width)])
                    .split(chunk)
            };

            let formatted_timestamp = msg.timestamp.format("%H:%M").to_string();

            let bubble_block = Block::default()
                .borders(Borders::ALL)
                .style(if msg.sender == 0 {
                    Style::default().fg(Color::Gray)
                } else {
                    Style::default().fg(Color::Cyan)
                })
                .title(if msg.sender == 0 {
                    Line::from(formatted_timestamp).left_aligned()
                } else {
                    Line::from(formatted_timestamp).right_aligned()
                });

            let paragraph = Paragraph::new(text).block(bubble_block);

            let position = if msg.sender == 0 { 0 } else { 1 }; // 0 -> left; 1 -> right
            frame.render_widget(paragraph, horizontal_layout[position]);
            frame.render_widget(&block, area);
        }
    }
}
