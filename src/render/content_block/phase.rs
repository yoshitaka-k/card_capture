use ratatui::{
    layout::{Rect},
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

    let text = match app.current_phase {
        GamePhase::Setup => "Setup phase: Draw cards from enemy deck",
        GamePhase::SetupEnd => "Setup end phase: End Setup phase",
        GamePhase::Enemy => "Enemy phase: Draw cards from enemy deck",
        GamePhase::Discard => "Discard phase: Player hand to discard pile",
        GamePhase::Draw => "Draw phase: Draw cards from player deck",
        GamePhase::Capture => "Capture phase: Select cards to capture enemy one and player multiple",
        GamePhase::End => "End phase: Next turn or end game",
    };

    let paragraph = Paragraph::new(text).block(block);

    frame.render_widget(paragraph, area);
}
