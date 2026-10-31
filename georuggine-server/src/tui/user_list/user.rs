use chrono::{DateTime, Utc};
use ratatui::{text::Line, widgets::ListItem};

use crate::tui::user_list::user_status::UserStatus;

#[derive(Clone, Default)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub status: UserStatus,
    pub last_timestamp: Option<DateTime<Utc>>,
    pub selected: bool,
}

impl User {
    pub fn new(id: u32, username: &str) -> Self {
        User {
            id,
            username: username.to_owned(),
            selected: false,
            status: UserStatus::Disconnected,
            last_timestamp: None,
        }
    }
}

impl From<&User> for ListItem<'_> {
    fn from(user: &User) -> Self {
        let prefix = if user.selected { "✗" } else { "☐" };
        ListItem::new(Line::styled(
            format!("{prefix} {}", user.username),
            user.status.to_color(),
        ))
    }
}
