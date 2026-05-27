use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Render the player content block
pub fn middle_content(app: &mut App, frame: &mut Frame, area: Rect) {
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

    let trash_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ]).split(player_chunks[1]);

    let player_discard_block = Block::default()
        .title("Player Discard")
        .title_style(Style::default().fg(Color::Blue).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .padding(Padding::horizontal(1));

    let player_discard_content = Paragraph::new(
        format!("Player Discard count: {}", app.game.get_player_discard().len())
    ).block(player_discard_block);

    app.positions.set_player_discard(trash_chunks[0]);
    frame.render_widget(player_discard_content, trash_chunks[0]);

    let enemy_trash_block = Block::default()
        .title("Enemy Discard")
        .title_style(Style::default().fg(Color::Red).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::horizontal(1));

    let enemy_trash_content = Paragraph::new(
        format!("Enemy Discard count: {}", app.game.get_enemy_discard().len())
    ).block(enemy_trash_block);

    app.positions.set_enemy_discard(trash_chunks[2]);
    frame.render_widget(enemy_trash_content, trash_chunks[2]);
}
