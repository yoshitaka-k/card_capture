use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Text,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use figlet_rs::FIGlet;

use crate::app::App;

pub fn render_gameover_block(app: &mut App, frame: &mut Frame, area: Rect) {
    app.positions.clear();

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

    // Game Over Title
    let figlet = FIGlet::standard().unwrap();
    let title = &format!("{}", figlet.convert("Game Over").unwrap());

    let paragraph = Paragraph::new(Text::from(title.clone()))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).bold());

    frame.render_widget(paragraph, content_chunks[1]);

    // Game Over Text
    let text = "You lose!\nClick to continue";
    let text_paragraph = Paragraph::new(Text::from(text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Magenta));

    frame.render_widget(text_paragraph, content_chunks[2]);
    app.positions.set_gameover(content_chunks[2]);
}
