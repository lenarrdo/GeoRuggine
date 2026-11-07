use std::fmt::Display;

use ratatui::style::Color;

#[derive(Clone, PartialEq)]
pub enum UserStatus {
    Moving,
    Stopped,
    Disconnected,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Stopped
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UserStatus::Disconnected => "Disconnected",
            UserStatus::Stopped => "Stopped",
            UserStatus::Moving => "Moving",
        };

        write!(f, "{str}")
    }
}

impl UserStatus {
    pub fn to_color(&self) -> Color {
        match self {
            UserStatus::Disconnected => Color::DarkGray,
            UserStatus::Stopped => Color::Gray,
            UserStatus::Moving => Color::LightGreen,
        }
    }
}
