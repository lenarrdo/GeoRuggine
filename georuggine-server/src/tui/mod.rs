use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Paragraph},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

const TEXT_FG_COLOR: Color = SLATE.c200;

pub enum AppEvent {
    Input(CrosstermEvent),
    ClientConnected(u32),
}

pub struct App {
    connected_users: Vec<String>,
    exit: bool,
    pub tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        App {
            connected_users: vec![],
            exit: false,
            tx,
            rx,
        }
    }

    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tx_input = self.tx.clone();
        std::thread::spawn(move || {
            loop {
                if let Ok(ev) = event::read() {
                    if tx_input.blocking_send(AppEvent::Input(ev)).is_err() {
                        break;
                    }
                }
            }
        });

        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            match self.rx.recv().await {
                Some(AppEvent::Input(CrosstermEvent::Key(key))) => self.handle_key(key),
                Some(AppEvent::ClientConnected(user_id)) => {
                    self.connected_users.push(format!("User{user_id}"));
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let [main_content, footer] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(frame.area());
        let [user_bar, info] =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)])
                .areas(main_content);
        let [chat, logs_stats] =
            Layout::vertical([Constraint::Percentage(75), Constraint::Fill(1)]).areas(info);
        let [logs, stats] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Fill(1)]).areas(logs_stats);

        let rounded_block = |title| {
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TEXT_FG_COLOR))
                .title(format!(" {title} "))
        };
        frame.render_widget(rounded_block("Users"), user_bar);
        frame.render_widget(rounded_block("Chat"), chat);
        frame.render_widget(rounded_block("Logs"), logs);
        frame.render_widget(rounded_block("Stats"), stats);
        frame.render_widget(
            Paragraph::new("Use ↓↑ to move, g/G to go top/bottom, q to quit."),
            footer,
        );
    }
}
