use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Constraint,
    widgets::{Block, Clear, Paragraph},
};

#[derive(Default)]
pub struct UserInfoPopup {
    pub visible: bool,
    pub avg_speed: f64,
}

impl UserInfoPopup {
    pub fn render(&self, block: Block, frame: &mut Frame) {
        if !self.visible {
            return;
        }

        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(40), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);

        let info = format!(
            "ID: {}\nUsername: {}\nAverage Speed: {} km/h\n",
            1, "test", self.avg_speed
        );
        let paragraph = Paragraph::new(info).block(block);

        frame.render_widget(paragraph, centered_area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.visible = false,
            _ => {}
        };
    }
}
