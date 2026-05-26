use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

/// Render the header block
pub fn render_header_block(frame: &mut Frame, area: Rect) {
    let header_block = Block::default()
        .padding(Padding::horizontal(1))
        .borders(Borders::ALL)
        .style(Style::default());

    let header = Paragraph::new(Text::styled(
        "Hello Ratatui App",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(header_block);

    frame.render_widget(header, area);
}
