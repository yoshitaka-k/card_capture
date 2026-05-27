use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;
use crate::trump::deck::DeckType;
use crate::render::content_block::hand_area_layout;

pub fn player_content(app: &mut App, frame: &mut Frame, area: Rect) {
    // Player Block
    let player_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]).split(area);

    // Deck Block
    let deck_block = Block::default()
        .title("Player Deck")
        .title_style(Style::default().fg(Color::Blue).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .padding(Padding::horizontal(1));

    let deck_content = Paragraph::new(
            format!("Player Deck count: {}", app.game.get_player_deck().len())
        ).block(deck_block);

    app.positions.set_player_deck(player_chunks[0]);
    frame.render_widget(deck_content, player_chunks[0]);

    // Hand Area Block
    hand_area_layout(app, frame, player_chunks[1], DeckType::Player);
}
