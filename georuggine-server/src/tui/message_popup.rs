use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Position},
    widgets::{Block, Clear, Paragraph},
};

use crate::tui::user_list::UserList;

#[derive(Default)]
pub struct MessagePopup {
    pub visible: bool,
    contents: String,
    character_index: usize,
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
        let scroll_x = if self.character_index >= popup_inner_width {
            (self.character_index - popup_inner_width + 1) as u16
        } else {
            0
        };

        let paragraph = Paragraph::new(self.contents.as_str())
            .block(block)
            .scroll((0, scroll_x));

        frame.render_widget(paragraph, centered_area);

        let cursor_x = centered_area.x + 1 + (self.character_index - scroll_x as usize) as u16;
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

    pub fn handle_key(&mut self, key: KeyEvent, user_list: &mut UserList) {
        match key.code {
            KeyCode::Char(c) => self.type_char(c),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::End => self.move_cursor_end(),
            KeyCode::Home => self.move_cursor_beginning(),
            KeyCode::Enter => {
                // TODO: send message
                self.hide_and_clear();
                user_list.clear_selected_users();
            }
            KeyCode::Esc => self.hide(),
            _ => {}
        }
    }

    pub fn type_char(&mut self, c: char) {
        self.contents.insert(self.character_index, c);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.character_index != 0 {
            self.contents.remove(self.character_index - 1);
            self.move_cursor_left();
        }
    }

    pub fn clear_input(&mut self) {
        self.contents.clear();
        self.character_index = 0;
    }

    pub fn move_cursor_right(&mut self) {
        let next_pos = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(next_pos);
    }

    pub fn move_cursor_left(&mut self) {
        let next_pos = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(next_pos);
    }

    pub fn move_cursor_end(&mut self) {
        self.character_index = self.contents.chars().count();
    }

    pub fn move_cursor_beginning(&mut self) {
        self.character_index = 0;
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.contents.chars().count())
    }
}
