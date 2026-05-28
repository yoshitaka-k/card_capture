use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::centered_rect;

/// Render the exit popup block
pub fn render_exit_popup_block(frame: &mut Frame) {
    let block = Block::default()
        .title("Quit App")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let text = Text::styled(
        "\n\nWould you like to quit the app? (y/n)",
        Style::default().fg(Color::Red),
    );

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(40, 14, frame.area());
    frame.render_widget(paragraph, area);
}
