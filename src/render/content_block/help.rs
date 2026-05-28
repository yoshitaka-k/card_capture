use ratatui::{
    layout::{Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Render the help content block
pub fn help_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Help Content")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let text = if app.help_text.is_empty() {
        "Help content: Help text"
    } else {
        &app.help_text
    };

    let content = Paragraph::new(text).block(block);

    frame.render_widget(content, area);
}
