use ratatui::Frame;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(
            format!("Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press `j` and `k` to increment and decrement the counter.\n\
                Counter: {}", app.counter)
        ).block(Block::default().title("Counter").borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Rounded)).style(Style::default().fg(Color::Yellow)).alignment(ratatui::layout::Alignment::Center),
        f.size(),
    )
}