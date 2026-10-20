use chrono::Duration;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Offset, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Clear, Paragraph, Tabs},
};

use crate::tui::user_list::User;

pub enum Tab {
    Daily,
    Weekly,
    Monthly,
}

impl Tab {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Tab::Daily),
            1 => Some(Tab::Weekly),
            2 => Some(Tab::Monthly),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct UserInfoPopup {
    // popup info
    pub visible: bool,
    selected_tab: usize,

    // selected user info
    pub user: User,
    pub avg_speed: f64,
    pub time_moving: Duration,
    pub time_not_moving: Duration,
}

impl UserInfoPopup {
    pub fn render(&self, frame: &mut Frame) {
        if !self.visible {
            return;
        }

        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(40), Constraint::Percentage(20));

        frame.render_widget(Clear, centered_area);

        self.render_content(frame, centered_area);
        self.render_tabs(frame, centered_area + Offset::new(1, 0));
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new(vec!["Daily Stats", "Weekly Stats", "Monthly Stats"])
            .style(Color::White)
            .highlight_style(Style::default().magenta().on_black().bold())
            .select(self.selected_tab)
            .divider(symbols::DOT)
            .padding(" ", " ");
        frame.render_widget(tabs, area);
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        let info = format!(
            "ID: {}\nUsername: {}\nStatus: {}\nAverage Speed: {:.2} km/h\nTime Moving: {}\nTime Not Moving: {}\n",
            self.user.id,
            self.user.username,
            self.user.status,
            self.avg_speed,
            self.time_moving.to_string(),
            self.time_not_moving.to_string()
        );
        let block = Paragraph::new(info)
            .alignment(Alignment::Left)
            .block(Block::bordered());
        frame.render_widget(block, area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> (u32, Option<Tab>) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.visible = false;
                return (self.user.id, None);
            }
            KeyCode::Char('l') | KeyCode::Right => self.selected_tab = (self.selected_tab + 1) % 3,
            KeyCode::Char('h') | KeyCode::Left => self.selected_tab = (self.selected_tab + 2) % 3,
            _ => {}
        };

        (self.user.id, Tab::from_index(self.selected_tab))
    }
}
