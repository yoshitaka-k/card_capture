use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};
use std::fmt::Write as _;

use crate::{app::App, constants::MAX_HAND_SIZE, trump::{deck::DeckType, Card}};

/// Render the content block
pub fn render_content_block(app: &mut App, frame: &mut Frame, area: Rect) {
    app.positions.clear();

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Min(3),
        ])
        .split(area);

    // Top Block
    enemy_content(app, frame, content_chunks[0]);

    // Middle Block
    player_content(app, frame, content_chunks[1]);

    // Bottom Block
    hand_content(app, frame, content_chunks[2]);

    // Help Block
    help_content(app, frame, content_chunks[3]);
}

fn hand_area_layout(app: &mut App, frame: &mut Frame, area: Rect, deck_type: DeckType) {
    let hand_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]).split(area);

    // Enemy Hand Block（右端から手札を並べる）
    for i in 0..MAX_HAND_SIZE {
        let chunk_index = MAX_HAND_SIZE - 1 - i;
        let chunk = hand_chunks[chunk_index];

        let hand_block = Block::default()
            .borders(Borders::ALL)
            .border_style(match deck_type {
                DeckType::Enemy => Style::default().fg(Color::Red),
                DeckType::Player => Style::default().fg(Color::Green),
            })
            .padding(Padding::horizontal(1))
            .style(Style::default());

        let text = match deck_type {
            DeckType::Enemy => {
                if let Some(card) = app.game.get_enemy_hand().get_card(i) {
                    if let Some(selected_card) = &app.game.get_enemy_select() {
                        if card.equals(selected_card) {
                            build_hand_text("Enemy Hand: ", Some(card), true)
                        } else {
                            build_hand_text("Enemy Hand: ", Some(card), false)
                        }
                    } else {
                        build_hand_text("Enemy Hand: ", Some(card), false)
                    }
                } else {
                    build_hand_text("Enemy Hand: ", None, false)
                }
            }
            DeckType::Player => {
                if let Some(card) = app.game.get_player_hand().get_card(i) {
                    if app.game.is_player_selected(i) {
                        build_hand_text("Player Hand: ", Some(card), true)
                    } else {
                        build_hand_text("Player Hand: ", Some(card), false)
                    }
                } else {
                    build_hand_text("Player Hand: ", None, false)
                }
            }
        };

        let hand_content = Paragraph::new(text).block(hand_block);

        match deck_type {
            DeckType::Enemy => {
                app.positions.add_enemy_hand(chunk);
            }
            DeckType::Player => {
                app.positions.add_player_hand(chunk);
            }
        }
        frame.render_widget(hand_content, chunk);
    }
}

fn build_hand_text(prefix: &str, card: Option<&Card>, selected: bool) -> String {
    let mut text = String::with_capacity(32);
    text.push_str(prefix);
    match card {
        Some(card) => {
            let _ = write!(&mut text, "{}", card);
            if selected {
                text.push_str("\nSelected");
            }
        }
        None => text.push_str("Empty"),
    }
    text
}

/// Render the enemy content block
fn enemy_content(app: &mut App, frame: &mut Frame, area: Rect) {
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

/// Render the player content block
fn player_content(app: &mut App, frame: &mut Frame, area: Rect) {
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

/// Render the hand content block
fn hand_content(app: &mut App, frame: &mut Frame, area: Rect) {
    hand_area_layout(app, frame, area, DeckType::Player);
}

/// Render the help content block
fn help_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let help_block = Block::default()
        .title("Help Content")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let help_content = Paragraph::new(app.help_text.as_str())
    .block(help_block);

    frame.render_widget(help_content, area);
}
