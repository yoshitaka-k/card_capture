use ratatui::{
    layout::{Rect, Direction, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::{App, GamePhase};

/// Render the phase content block
pub fn phase_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Current Phase")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner);

    let paragraph = Paragraph::new(match app.current_phase {
        GamePhase::Setup => "Setup phase: Enemy draw cards from enemy deck",
        GamePhase::SetupEnd => "Setup end phase: End Setup phase",
        GamePhase::Enemy => "Enemy phase: Enemy draw cards from enemy deck",
        GamePhase::Discard => "Player Discard phase: Player hand to discard pile",
        GamePhase::Draw => "Player Draw phase: Player draw cards from player deck",
        GamePhase::Capture => "Capture phase: Select cards to capture enemy one and player multiple",
        GamePhase::End => "End phase: Next turn or end game",
    });

    frame.render_widget(paragraph, inner_chunks[1]);
}
