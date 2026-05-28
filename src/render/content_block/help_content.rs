use ratatui::{
    layout::{Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::{App, GamePhase};

/// Render the help content block
pub fn help_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let help_block = Block::default()
        .title("Help Content")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let help_text = if app.help_text.is_empty() {
        match app.current_phase {
            GamePhase::Initial => "Initial phase: Draw cards from enemy deck",
            GamePhase::InitialEnd => "Initial end phase: End Initial phase",
            GamePhase::Enemy => "Enemy phase: Draw cards from enemy deck",
            GamePhase::Discard => "Discard phase: Player hand to discard pile",
            GamePhase::Draw => "Draw phase: Draw cards from player deck",
            GamePhase::Battle => "Battle phase: Select cards to battle enemy one and player multiple",
            GamePhase::End => "End phase: Next turn or end game",
        }
    } else {
        &app.help_text
    };

    let help_content = Paragraph::new(help_text).block(help_block);

    frame.render_widget(help_content, area);
}
