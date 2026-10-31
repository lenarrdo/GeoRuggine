use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Position},
    widgets::{Block, Clear, Paragraph},
};

#[derive(Default)]
pub struct MessagePopup {
    pub visible: bool,
    input: Vec<char>,
    cursor_position: usize,
}

impl MessagePopup {
    pub fn render(&self, block: Block, frame: &mut Frame) {
        if !self.visible {
            return;
        }

        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(40), Constraint::Length(3));
        frame.render_widget(Clear, centered_area);

        let popup_inner_width = (centered_area.width.saturating_sub(2)) as usize;
        let scroll_x = if self.cursor_position >= popup_inner_width {
            (self.cursor_position - popup_inner_width + 1) as u16
        } else {
            0
        };

        let contents: String = self.input.iter().collect();
        let paragraph = Paragraph::new(contents).block(block).scroll((0, scroll_x));

        frame.render_widget(paragraph, centered_area);

        let cursor_x = centered_area.x + 1 + (self.cursor_position - scroll_x as usize) as u16;
        frame.set_cursor_position(Position::new(cursor_x, centered_area.y + 1));
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn hide_and_clear(&mut self) {
        self.clear_input();
        self.hide();
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        if let KeyCode::Enter = key.code {
            let contents = self.input.clone();
            self.hide_and_clear();
            return Some(contents.iter().collect());
        }

        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Backspace) => self.delete_char(),
            (KeyModifiers::CONTROL, KeyCode::Char('w')) => self.delete_word(),
            (KeyModifiers::NONE, KeyCode::Left) => self.move_cursor_left(),
            (KeyModifiers::NONE, KeyCode::Right) => self.move_cursor_right(),
            (KeyModifiers::NONE, KeyCode::End) => self.move_cursor_end(),
            (KeyModifiers::NONE, KeyCode::Home) => self.move_cursor_beginning(),
            (KeyModifiers::NONE, KeyCode::Esc) => self.hide(),
            (_, KeyCode::Char(c)) => self.type_char(c),
            _ => {}
        };

        None
    }

    pub fn type_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position != 0 {
            self.input.remove(self.cursor_position - 1);
            self.move_cursor_left();
        }
    }

    pub fn delete_word(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        let mut idx = self.cursor_position;

        while idx > 0 && self.input[idx - 1].is_whitespace() {
            idx -= 1;
        }

        while idx > 0 && !self.input[idx - 1].is_whitespace() {
            idx -= 1;
        }

        self.input.drain(idx..self.cursor_position);
        self.cursor_position = idx;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn move_cursor_right(&mut self) {
        let next_pos = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(next_pos);
    }

    pub fn move_cursor_left(&mut self) {
        let next_pos = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(next_pos);
    }

    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input.iter().count();
    }

    pub fn move_cursor_beginning(&mut self) {
        self.cursor_position = 0;
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.iter().count())
    }
}
