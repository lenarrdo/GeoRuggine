pub mod chat_box;
pub mod logs_tab;
pub mod message_popup;
pub mod user_info_popup;
pub mod user_list;

use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use crossterm::event::{
    Event as CrosstermEvent, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use futures::StreamExt;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Paragraph},
};
use tokio::sync::mpsc::Receiver;

use crate::{
    auth::user_repository::UserEntity,
    messages::{
        position::repository::PositionRepository,
        text::{TextMessage, repository::TextMessageRepository},
    },
    shared::{app_state::AppState, utils::SERVER_ID},
    tui::{
        chat_box::{ChatBox, ui_message::UIMessage},
        logs_tab::LogsTab,
        message_popup::MessagePopup,
        user_info_popup::UserInfoPopup,
        user_list::{UserList, user::User, user_status::UserStatus},
    },
};

const TEXT_FG_COLOR: Color = SLATE.c200;

pub enum TimeFrame {
    Today,
    ThisWeek,
    ThisMonth,
}

impl TimeFrame {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(TimeFrame::Today),
            1 => Some(TimeFrame::ThisWeek),
            2 => Some(TimeFrame::ThisMonth),
            _ => None,
        }
    }
}

pub enum AppEvent {
    UserRegistered((u32, String)),
    UserConnected(u32),
    UserDisconnected(u32),
    UserMoving(u32),
    UserStopped(u32),
    LogMessage(String),
    NewMessage((u32, TextMessage)),
    Tick,
}

pub struct App {
    // exit condition
    exit: bool,

    // widgets
    user_list: UserList,
    message_popup: MessagePopup,
    user_info_popup: UserInfoPopup,
    logs_tab: LogsTab,
    chat_box: ChatBox,

    // domain info
    rx: Receiver<AppEvent>,
    state: Arc<AppState>,
}

impl App {
    pub fn new(
        rx: Receiver<AppEvent>,
        user_entities: Vec<UserEntity>,
        state: Arc<AppState>,
    ) -> Self {
        let users = user_entities
            .iter()
            .map(|user_entity| User::new(user_entity.id, &user_entity.username))
            .collect();

        App {
            exit: false,
            user_list: UserList::new(users),
            message_popup: MessagePopup::default(),
            user_info_popup: UserInfoPopup::default(),
            logs_tab: LogsTab::default(),
            chat_box: ChatBox::default(),
            rx,
            state,
        }
    }

    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = EventStream::new();

        let tx = self.state.tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let _ = tx.send(AppEvent::Tick).await;
            }
        });

        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            tokio::select! {
                Some(Ok(crossterm_event)) = reader.next() => {
                    if let CrosstermEvent::Key(key) = crossterm_event && key.kind == KeyEventKind::Press {
                        self.handle_key(key).await
                    }
                }
                Some(app_event) = self.rx.recv() => {
                    match app_event {
                        AppEvent::UserRegistered((user_id, username)) => self.user_list.add_user(User::new(user_id, &username)),
                        AppEvent::UserConnected(user_id) => {
                            self.user_list.set_status(user_id, UserStatus::Still);
                            self.logs_tab.add_message(format!("User #{user_id} connected."));
                        }
                        AppEvent::UserDisconnected(user_id) => {
                            self.user_list.set_status(user_id, UserStatus::Disconnected);
                            self.logs_tab.add_message(format!("User #{user_id} disconnected."));
                        },
                        AppEvent::UserMoving(user_id) => self.user_list.set_status(user_id, UserStatus::Moving),
                        AppEvent::UserStopped(user_id) => self.user_list.set_status(user_id, UserStatus::Still),
                        AppEvent::LogMessage(msg) => self.logs_tab.add_message(msg),
                        AppEvent::Tick => self.user_list.scan_inactive(),
                        AppEvent::NewMessage((sender_id, text_msg)) => {
                            self.update_chat_box(sender_id, &text_msg);
                            self.logs_tab.add_message(format!("New message from User #{sender_id}: {}", text_msg.contents));
                        },
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        if self.message_popup.visible {
            let Some(msg) = self.message_popup.handle_key(key) else {
                return;
            };
            let text_msg = TextMessage::new(msg.clone());

            let previously_selected = self.user_list.clear_selected_users();
            if previously_selected.is_empty() {
                self.logs_tab
                    .add_message("You didn't select any user. No messages were sent.".to_string());
                return;
            }

            for receiver_id in previously_selected {
                if let Err(e) = self.state.clients.send_one(receiver_id, &msg).await {
                    self.logs_tab.add_message(e);
                    continue;
                }

                if let Some(journey_id) = self.state.clients.get_journey_id(receiver_id) {
                    let res = TextMessageRepository::new(self.state.db.clone())
                        .save_message(SERVER_ID, receiver_id, journey_id, &text_msg)
                        .await;
                    self.logs_tab.add_message(format!("{res:?}"));
                }
            }

            // FIXME
            self.update_chat_box(SERVER_ID, &text_msg);

            return;
        } else if self.user_info_popup.visible {
            let (user_id, time_frame_opt) = self.user_info_popup.handle_key(key);
            let Some(time_frame) = time_frame_opt else {
                return;
            };

            let Ok(positions) = PositionRepository::new(self.state.db.clone())
                .get_positions(user_id, time_frame)
                .await
            else {
                return;
            };
            return self.user_info_popup.update_movement_stats(&positions);
        }

        if key.modifiers == KeyModifiers::SHIFT && key.code == KeyCode::Up {
            self.chat_box.scroll_up();
        } else if key.modifiers == KeyModifiers::SHIFT && key.code == KeyCode::Down {
            self.chat_box.scroll_down();
        } else {
            match key.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('k') | KeyCode::Up => {
                    self.user_list.select_prev();
                    if let Some(user) = self.user_list.get_hovered_user() {
                        self.chat_box.messages = self.load_user_messages(user.id).await;
                    }
                }
                KeyCode::Char('K') => self.chat_box.scroll_up(),
                KeyCode::Char('j') | KeyCode::Down => {
                    self.user_list.select_next();
                    if let Some(user) = self.user_list.get_hovered_user() {
                        self.chat_box.messages = self.load_user_messages(user.id).await;
                    }
                }
                KeyCode::Char('J') => self.chat_box.scroll_down(),
                KeyCode::Char('g') | KeyCode::Home => self.user_list.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.user_list.select_last(),
                KeyCode::Char(' ') => self.user_list.toggle_selected(),
                KeyCode::Char('a') => self.user_list.select_all_toggle(),
                KeyCode::Char('i') => {
                    let Some(user) = self.user_list.get_hovered_user() else {
                        return;
                    };

                    let positions = PositionRepository::new(self.state.db.clone())
                        .get_positions(user.id, TimeFrame::Today)
                        .await
                        .unwrap_or_default(); // improve handling
                    self.user_info_popup.update_movement_stats(&positions);
                    self.user_info_popup.user = user;

                    self.user_info_popup.visible = true
                }
                KeyCode::Char('n') => self.message_popup.show(),
                _ => {}
            }
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

        let rounded_block = |title| {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TEXT_FG_COLOR))
                .title(format!(" {title} "))
        };

        frame.render_widget(
            Paragraph::new("Use ↓↑ to move, g/G to go top/bottom, q to quit."),
            footer,
        );

        self.user_list
            .render(rounded_block("Users"), frame, user_bar);

        self.chat_box.render(rounded_block("Chat"), frame, chat);

        self.logs_tab
            .render(rounded_block("Logs"), frame, logs_stats);

        self.user_info_popup.render(frame);

        self.message_popup
            .render(rounded_block("New Message"), frame);
    }

    async fn load_user_messages(&self, user_id: u32) -> Vec<UIMessage> {
        let Some(journey_id) = self.state.clients.get_journey_id(user_id) else {
            return vec![];
        };

        let res = TextMessageRepository::new(self.state.db.clone())
            .get_messages(journey_id)
            .await;
        if let Err(e) = res {
            vec![UIMessage::new(420, &e.to_string(), Utc::now())]
        } else {
            let message_rows = res.unwrap();
            message_rows
                .into_iter()
                .map(|msg| UIMessage::new(msg.sender_id, &msg.contents, msg.timestamp))
                .collect()
        }
    }

    fn update_chat_box(&mut self, sender_id: u32, text_msg: &TextMessage) {
        if let Some(hovered_user) = self.user_list.get_hovered_user()
            && (hovered_user.id == sender_id || sender_id == SERVER_ID)
        {
            let date_time: DateTime<Utc> = text_msg.timestamp.parse().unwrap_or(Utc::now());
            self.chat_box
                .messages
                .push(UIMessage::new(sender_id, &text_msg.contents, date_time));
        }
    }
}
