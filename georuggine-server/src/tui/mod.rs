use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, palette::tailwind::SLATE},
    widgets::{Block, BorderType, Paragraph},
};

const TEXT_FG_COLOR: Color = SLATE.c200;

pub struct App {
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App { exit: false }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
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
