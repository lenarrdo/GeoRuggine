use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Offset, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Clear, Paragraph, Tabs},
};

use crate::tui::user_list::User;

#[derive(Default)]
pub struct UserInfoPopup {
    // popup info
    pub visible: bool,
    selected_tab: usize,

    // selected user info
    pub user: User,
    pub avg_speed: f64,
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
            "ID: {}\nUsername: {}\nStatus: {}\nAverage Speed: {:.2} km/h\n",
            self.user.id, self.user.username, self.user.status, self.avg_speed
        );
        let text = match self.selected_tab {
            0 => &info,
            1 => "todo!()",
            2 => "todo!()",
            _ => unreachable!(),
        };
        let block = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(Block::bordered());
        frame.render_widget(block, area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.visible = false,
            KeyCode::Char('l') | KeyCode::Right => self.selected_tab = (self.selected_tab + 1) % 3,
            KeyCode::Char('h') | KeyCode::Left => self.selected_tab = (self.selected_tab + 2) % 3,
            _ => {}
        };
    }
}
