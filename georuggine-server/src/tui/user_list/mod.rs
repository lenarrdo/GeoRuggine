pub mod user;
pub mod user_status;

use chrono::Utc;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, List, ListItem, ListState},
};

use crate::{
    shared::utils::INACTIVE_THRESHOLD,
    tui::user_list::{user::User, user_status::UserStatus},
};

#[derive(Default)]
pub struct UserList {
    users: Vec<User>,
    state: ListState,
}

impl UserList {
    pub fn new(users: Vec<User>) -> Self {
        let mut state = ListState::default();
        if !users.is_empty() {
            state.select(Some(0));
        }

        UserList { users, state }
    }

    pub fn render(&mut self, block: Block, frame: &mut Frame, area: Rect) {
        let list = List::new(
            self.users
                .iter()
                .map(|user| ListItem::from(user))
                .collect::<Vec<_>>(),
        )
        .block(block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn select_next(&mut self) {
        if let Some(curr_idx) = self.state.selected()
            && !self.users.is_empty()
        {
            let next_idx = (curr_idx + 1) % self.users.len();
            self.state.select(Some(next_idx));
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(curr_idx) = self.state.selected()
            && !self.users.is_empty()
        {
            let prev_idx = if curr_idx == 0 {
                self.users.len() - 1
            } else {
                curr_idx - 1
            };

            self.state.select(Some(prev_idx));
        }
    }

    pub fn select_first(&mut self) {
        self.state.select_first()
    }

    pub fn select_last(&mut self) {
        self.state.select_last()
    }

    pub fn select_all_toggle(&mut self) {
        let online_clients: Vec<&mut User> = self
            .users
            .iter_mut()
            .filter(|user| user.status != UserStatus::Disconnected)
            .collect();

        let all_selected =
            online_clients.iter().filter(|user| user.selected).count() == online_clients.len();

        online_clients
            .into_iter()
            .for_each(|user| user.selected = !all_selected);
    }

    pub fn toggle_selected(&mut self) {
        if let Some(i) = self.state.selected() {
            self.users[i].selected = !self.users[i].selected;
            // self.select_next();
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
        if self.state.selected().is_none() {
            self.state.select_first();
        }
    }

    /// sets every user as unselected.
    /// returns a vector containing the ids of the previously selected users.
    pub fn clear_selected_users(&mut self) -> Vec<u32> {
        let mut previously_selected = vec![];
        for user in &mut self.users {
            if user.selected {
                previously_selected.push(user.id);
                user.selected = false;
            }
        }

        previously_selected
    }

    pub fn set_status(&mut self, user_id: u32, new_status: UserStatus) {
        let Some(user) = self.users.iter_mut().find(|user| user.id == user_id) else {
            return;
        };

        if new_status != UserStatus::Disconnected {
            user.last_timestamp = Some(Utc::now());
        }
        user.status = new_status;
    }

    pub fn scan_inactive(&mut self) {
        for user in &mut self.users {
            if user.status == UserStatus::Disconnected {
                continue;
            }

            if let Some(timestamp) = user.last_timestamp
                && Utc::now().signed_duration_since(timestamp) > INACTIVE_THRESHOLD
            {
                user.status = UserStatus::Still;
            }
        }
    }

    pub fn get_hovered_user(&mut self) -> Option<User> {
        if let Some(idx) = self.state.selected() {
            return Some(self.users[idx].clone());
        }

        None
    }
}
