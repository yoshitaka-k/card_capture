use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;
use crate::trump::deck::DeckType;
use crate::render::content_block::hand_area_layout;

/// Render the enemy content block
pub fn enemy_content(app: &mut App, frame: &mut Frame, area: Rect) {
    // Enemy Block
    let enemy_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]).split(area);

    // Deck Block
    let deck_block = Block::default()
        .title("Enemy Deck")
        .title_style(Style::default().fg(Color::Red).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::horizontal(1));

    let deck_content = Paragraph::new(
            format!("Enemy Deck count: {}", app.game.get_enemy_deck().len())
        ).block(deck_block);

    app.positions.set_enemy_deck(enemy_chunks[0]);
    frame.render_widget(deck_content, enemy_chunks[0]);

    hand_area_layout(app, frame, enemy_chunks[1], DeckType::Enemy);
}
