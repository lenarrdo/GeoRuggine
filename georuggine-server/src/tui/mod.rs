use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Paragraph},
};
use tokio::sync::mpsc;

const TEXT_FG_COLOR: Color = SLATE.c200;

pub enum AppEvent {
    Input(CrosstermEvent),
    ClientDisconnected(u32),
    Tick,
}

pub struct App {
    exit: bool,
    connected_users: Vec<u32>,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            connected_users: vec![],
        }
    }

    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, mut rx) = mpsc::channel(100);

        let tx_input = tx.clone();
        std::thread::spawn(move || {
            loop {
                if let Ok(ev) = event::read() {
                    if tx_input.blocking_send(AppEvent::Input(ev)).is_err() {
                        break;
                    }
                }
            }
        });

        let tx_clients = tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                if tx_clients.send(AppEvent::Tick).await.is_err() {
                    break;
                }
            }
        });

        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            match rx.recv().await {
                Some(AppEvent::Input(CrosstermEvent::Key(key))) => self.handle_key(key),
                Some(AppEvent::Tick) => {}
                Some(AppEvent::ClientDisconnected(user_id)) => {
                    self.connected_users.retain(|&x| x != user_id)
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
