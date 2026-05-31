use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::{App, CurrentScreen};

/// Render the footer block
pub fn render_footer_block(app: &App, frame: &mut Frame, area: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Current Navigation Text
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Game Playing", Style::default().fg(Color::Green).bold()),
            CurrentScreen::GameClear => Span::styled("Game Clear", Style::default().fg(Color::Green).bold()),
            CurrentScreen::GameOver => Span::styled("Game Over", Style::default().fg(Color::Red).bold()),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed).bold()),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            Span::styled(format!("Turn: {}", app.turn), Style::default().fg(Color::DarkGray))
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().padding(Padding::horizontal(1)).borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Exiting => Span::styled(
                "(y) or (Enter) to yes, (n) or (Esc) to no",
                Style::default().fg(Color::Red).bold(),
            ),
            _ => Span::styled(
                "(q) to exit",
                Style::default().fg(Color::Red).bold(),
            )
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().padding(Padding::horizontal(1)).borders(Borders::ALL));

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}
