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
    // frame.render_widget(Clear, frame.area());

    let popup_block = Block::default()
        .title("Quit App")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let ext_text = Text::styled(
        "\n\nWould you like to quit the app? (y/n)",
        Style::default().fg(Color::Red),
    );

    let ext_paragraph = Paragraph::new(ext_text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let popup_area = centered_rect(40, 14, frame.area());
    frame.render_widget(ext_paragraph, popup_area);
}
