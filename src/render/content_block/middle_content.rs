use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;
use crate::trump::deck::DeckType;

/// Render the player content block
pub fn middle_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let trash_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ]).split(area);

    discard_content(app, frame, trash_chunks[0], DeckType::Player);

    // Battle Area Block
    let middle_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]).split(trash_chunks[1]);

    let battle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(40),
        ]).split(middle_chunks[1]);

    // Player Content Block
    let player_content = Paragraph::new(format!("Player: {}", app.game.calc_player_select_rank()))
        .style(Style::default().fg(Color::Blue).bold())
        .block(Block::default().padding(Padding::horizontal(1)))
        .alignment(Alignment::Center);
    frame.render_widget(player_content, battle_chunks[0]);

    // Middle Content Block
    let middle_content = Paragraph::new("VS")
        .style(Style::default().fg(Color::Yellow).bold())
        .block(Block::default().padding(Padding::horizontal(1)))
        .alignment(Alignment::Center);
    frame.render_widget(middle_content, battle_chunks[1]);

    // Enemy Content Block
    let enemy_content = Paragraph::new(format!("Enemy: {}", app.game.calc_enemy_select_rank()))
        .style(Style::default().fg(Color::Magenta).bold())
        .block(Block::default().padding(Padding::horizontal(1)))
        .alignment(Alignment::Center);
    frame.render_widget(enemy_content, battle_chunks[2]);

    discard_content(app, frame, trash_chunks[2], DeckType::Enemy);
}

/// Render the player discard content block
fn discard_content(app: &mut App, frame: &mut Frame, area: Rect, deck_type: DeckType) {
    let title: String = match deck_type {
        DeckType::Player => "Player Discard".to_string(),
        DeckType::Enemy => "Enemy Discard".to_string(),
    };

    // Player Discard Block
    let discard_block = Block::default()
        .title(title)
        .title_style(Style::default().fg(Color::Blue).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .padding(Padding::horizontal(1));

    let discard_content = Paragraph::new(match deck_type {
        DeckType::Player => {
            format!(
                "Player Discard count: {}\nCupture: {}\nDiscard: {}",
                app.game.get_player_discard().len(),
                app.game.is_player_cupture(),
                app.game.is_discard(),
            )
        }
        DeckType::Enemy => {
            format!(
                "Enemy Discard count: {}\nDiscard: {}\nSacrifice: {}",
                app.game.get_enemy_discard().len(),
                app.game.is_enemy_cupture(),
                app.game.is_sacrifice(),
            )
        }
    }).block(discard_block);

    match deck_type {
        DeckType::Player => app.positions.set_player_discard(area),
        DeckType::Enemy => app.positions.set_enemy_discard(area),
    }
    frame.render_widget(discard_content, area);
}
