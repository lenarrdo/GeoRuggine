pub mod message_popup;
pub mod user_list;

use std::sync::Arc;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEvent};
use futures::StreamExt;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Paragraph},
};
use tokio::sync::mpsc::Receiver;

use crate::{
    tui::{
        message_popup::MessagePopup,
        user_list::{User, UserList},
    },
    websocket::clients::ClientsList,
};

const TEXT_FG_COLOR: Color = SLATE.c200;

pub enum AppEvent {
    ClientConnected(User),
}

pub struct App {
    exit: bool,
    connected_users: UserList,
    message_popup: MessagePopup,
    rx: Receiver<AppEvent>,
    clients: Arc<ClientsList>,
}

impl App {
    pub fn new(rx: Receiver<AppEvent>, clients: Arc<ClientsList>) -> Self {
        App {
            exit: false,
            connected_users: UserList::default(),
            message_popup: MessagePopup::default(),
            rx,
            clients,
        }
    }

    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = EventStream::new();

        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            tokio::select! {
                Some(Ok(CrosstermEvent::Key(key))) = reader.next() => {
                    self.handle_key(key).await;
                }
                Some(app_event) = self.rx.recv() => {
                    match app_event {
                        AppEvent::ClientConnected(user) => {
                            self.connected_users.add_user(user);
                        }
                        // _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        if self.message_popup.visible {
            let msg = self.message_popup.handle_key(key);
            if msg.is_some() {
                let msg = msg.unwrap();
                let previously_selected = self.connected_users.clear_selected_users();
                for user_id in previously_selected {
                    self.clients.send_one(user_id, &msg).await;
                }
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('k') | KeyCode::Up => self.connected_users.select_prev(),
            KeyCode::Char('j') | KeyCode::Down => self.connected_users.select_next(),
            KeyCode::Char('g') | KeyCode::Home => self.connected_users.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.connected_users.select_last(),
            KeyCode::Char(' ') => self.connected_users.toggle_selected(),
            KeyCode::Char('n') => self.message_popup.show(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let [main_content, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let [user_bar, info] =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)])
                .areas(main_content);
        let [chat, logs_stats] =
            Layout::vertical([Constraint::Percentage(75), Constraint::Fill(1)]).areas(info);
        let [logs, stats] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Fill(1)]).areas(logs_stats);

        let rounded_block = |title| {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TEXT_FG_COLOR))
                .title(format!(" {title} "))
        };
        self.connected_users
            .render(rounded_block("Users"), frame, user_bar);
        frame.render_widget(rounded_block("Chat"), chat);
        frame.render_widget(rounded_block("Logs"), logs);
        frame.render_widget(rounded_block("Stats"), stats);
        frame.render_widget(
            Paragraph::new("Use ↓↑ to move, g/G to go top/bottom, q to quit."),
            footer,
        );

        self.message_popup
            .render(rounded_block("New Message"), frame);
    }
}
