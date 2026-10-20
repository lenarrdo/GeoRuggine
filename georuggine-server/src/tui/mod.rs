pub mod message_popup;
pub mod user_info_popup;
pub mod user_list;

use std::{sync::Arc, time::Duration};

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEvent};
use futures::StreamExt;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Paragraph},
};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    auth::user_repository::UserEntity,
    tui::{
        message_popup::MessagePopup,
        user_info_popup::UserInfoPopup,
        user_list::{User, UserList, UserStatus},
    },
    websocket::clients::ClientsList,
};

const TEXT_FG_COLOR: Color = SLATE.c200;

pub enum AppEvent {
    ClientConnected(u32),
    ClientDisconnected(u32),
    UserMoving(u32),
    UserStopped(u32),
    Tick,
}

pub struct App {
    // exit condition
    exit: bool,

    // widgets
    user_list: UserList,
    message_popup: MessagePopup,
    user_info_popup: UserInfoPopup,

    // communication
    tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,

    // domain info
    clients: Arc<ClientsList>,
}

impl App {
    pub fn new(
        tx: Sender<AppEvent>,
        rx: Receiver<AppEvent>,
        _user_entities: Vec<UserEntity>,
        clients: Arc<ClientsList>,
    ) -> Self {
        // let users = user_entities
        //     .iter()
        //     .map(|user_entity| User::new(user_entity.id, &user_entity.username))
        //     .collect();

        App {
            exit: false,
            // user_list: UserList::new(users),
            // user_list: UserList::default(),
            user_list: UserList::new(vec![
                User::new(1, "User 1"),
                User::new(2, "User 2"),
                User::new(3, "User 3"),
            ]),
            message_popup: MessagePopup::default(),
            user_info_popup: UserInfoPopup::default(),
            tx,
            rx,
            clients,
        }
    }

    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = EventStream::new();

        let tx = self.tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                tx.send(AppEvent::Tick).await.unwrap();
            }
        });

        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            tokio::select! {
                Some(Ok(crossterm_event)) = reader.next() => {
                    match crossterm_event {
                        CrosstermEvent::Key(key) =>self.handle_key(key).await,
                        _ => {}
                    }

                }
                Some(app_event) = self.rx.recv() => {
                    match app_event {
                        AppEvent::ClientConnected(user_id) => self.user_list.set_status(user_id, UserStatus::Still),
                        AppEvent::ClientDisconnected(user_id) => self.user_list.set_status(user_id, UserStatus::Disconnected),
                        AppEvent::UserMoving(user_id) => self.user_list.set_status(user_id, UserStatus::Moving),
                        AppEvent::UserStopped(user_id) => self.user_list.set_status(user_id, UserStatus::Still),
                        AppEvent::Tick => self.user_list.scan_inactive(),
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
                let previously_selected = self.user_list.clear_selected_users();
                for user_id in previously_selected {
                    self.clients.send_one(user_id, &msg).await;
                }
            }
            return;
        } else if self.user_info_popup.visible {
            return self.user_info_popup.handle_key(key);
        }

        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('k') | KeyCode::Up => self.user_list.select_prev(),
            KeyCode::Char('j') | KeyCode::Down => self.user_list.select_next(),
            KeyCode::Char('g') | KeyCode::Home => self.user_list.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.user_list.select_last(),
            KeyCode::Char(' ') => self.user_list.toggle_selected(),
            KeyCode::Char('a') => self.user_list.select_all_toggle(),
            KeyCode::Char('i') => {
                if let Some(user) = self.user_list.get_hovered_user() {
                    self.user_info_popup.avg_speed = self
                        .clients
                        .get_avg_speed(user.id)
                        .await
                        .unwrap_or_default();
                    self.user_info_popup.user = user;

                    self.user_info_popup.visible = true
                }
            }
            KeyCode::Char('n') => self.message_popup.show(),
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let [main_content, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(1)]).areas(frame.area());
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
        self.user_list
            .render(rounded_block("Users"), frame, user_bar);
        frame.render_widget(rounded_block("Chat"), chat);
        frame.render_widget(rounded_block("Logs"), logs);
        frame.render_widget(rounded_block("Stats"), stats);
        frame.render_widget(
            Paragraph::new("Use ↓↑ to move, g/G to go top/bottom, q to quit."),
            footer,
        );

        self.user_info_popup.render(frame);

        self.message_popup
            .render(rounded_block("New Message"), frame);
    }
}
