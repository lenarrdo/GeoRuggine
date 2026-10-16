use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};

pub enum UserStatus {
    Moving,
    Still,
}

impl UserStatus {
    pub fn to_color(&self) -> Color {
        match self {
            UserStatus::Still => Color::Gray,
            UserStatus::Moving => Color::LightGreen,
        }
    }
}

pub struct User {
    pub id: u32,
    pub username: String,
    selected: bool,
    status: UserStatus,
}

impl User {
    pub fn new(id: u32, username: &str) -> Self {
        User {
            id,
            username: username.to_owned(),
            selected: false,
            // new users are added the moment they send a message,
            // meaning that they are online and moving.
            status: UserStatus::Moving,
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

#[derive(Default)]
pub struct UserList {
    pub users: Vec<User>,
    pub state: ListState,
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
        if self.users.is_empty() {
            return;
        }

        let curr_idx = self.state.selected().unwrap();
        let next_idx = if curr_idx == self.users.len() - 1 {
            Some(0)
        } else {
            Some(curr_idx + 1)
        };

        self.state.select(next_idx);
    }

    pub fn select_prev(&mut self) {
        if self.users.is_empty() {
            return;
        }

        let curr_idx = self.state.selected().unwrap();
        let next_idx = if curr_idx == 0 {
            Some(self.users.len() - 1)
        } else {
            Some(curr_idx - 1)
        };

        self.state.select(next_idx);
    }

    pub fn select_first(&mut self) {
        self.state.select_first()
    }

    pub fn select_last(&mut self) {
        self.state.select_last()
    }

    pub fn toggle_selected(&mut self) {
        if let Some(i) = self.state.selected() {
            self.users[i].selected = !self.users[i].selected;
            self.select_next();
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

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
        if self.users.len() == 1 {
            self.state.select(Some(0));
        }
    }
}
