use ratatui::{
    layout::{Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Render the help content block
pub fn help_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let help_block = Block::default()
        .title("Help Content")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let help_content = Paragraph::new(app.help_text.as_str())
    .block(help_block);

    frame.render_widget(help_content, area);
}
