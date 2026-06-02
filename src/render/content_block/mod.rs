/// Enemy Content Block
pub mod enemy;
/// Middle Content Block
pub mod middle;
/// Player Content Block
pub mod player;
/// Help Content Block
pub mod help;
/// Phase Content Block
pub mod phase;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;
use crate::trump::deck::DeckType;
use crate::constants::MAX_HAND_SIZE;
use crate::trump::Card;
use std::fmt::Write as _;

/// 見た目のインデックスを実データのインデックスに変換する
#[inline]
fn visual_to_hand_index(visual_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - visual_index
}

/// 手札テキストを構築する
#[inline]
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

/// 手札エリアのレイアウトを行う
fn hand_area_layout(app: &mut App, frame: &mut Frame, area: Rect, deck_type: DeckType) {
    let hand_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25); 4]).split(area);

    // 見た目は左->右で走査し、実データ（手札）は右詰め対応の index に変換する。
    for visual_index in 0..MAX_HAND_SIZE {
        let hand_index = visual_to_hand_index(visual_index);
        let chunk = hand_chunks[visual_index];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(match deck_type {
                DeckType::Enemy => Style::default().fg(Color::Magenta),
                DeckType::Player => Style::default().fg(Color::Cyan),
            })
            .padding(Padding::horizontal(1))
            .style(Style::default());

        let text = match deck_type {
            DeckType::Enemy => {
                if let Some(card) = app.game.enemy_hand().card(hand_index) {
                    if app.game.is_enemy_selected(hand_index) {
                        build_hand_text("Enemy Hand: ", Some(card), true)
                    } else {
                        build_hand_text("Enemy Hand: ", Some(card), false)
                    }
                } else {
                    build_hand_text("Enemy Hand: ", None, false)
                }
            }
            DeckType::Player => {
                let mut text = if let Some(card) = app.game.player_hand().card(hand_index) {
                    if app.game.is_player_selected(hand_index) {
                        build_hand_text("Player Hand: ", Some(card), true)
                    } else {
                        build_hand_text("Player Hand: ", Some(card), false)
                    }
                } else {
                    build_hand_text("Player Hand: ", None, false)
                };

                if let Some(card) = app.game.player_hand().card(hand_index) {
                    if card.is_joker() {
                        if let Some(copy_label) = app
                            .game
                            .player_hand_copy_joker_source(card.rank())
                            .and_then(|index| {
                                app.game
                                    .player_hand()
                                    .card(index)
                                    .map(|card| format!("Copy from: {}", card.name()))
                            })
                        {
                            text.push('\n');
                            text.push_str(&copy_label);
                        }
                    }
                }

                text
            }
        };

        let paragraph = Paragraph::new(text).block(block);

        match deck_type {
            DeckType::Enemy => {
                app.positions.add_enemy_hand(chunk);
                frame.render_widget(paragraph, chunk);
            }
            DeckType::Player => {
                let trump_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(100),
                    Constraint::Length(3),
                ]).split(chunk);

                app.positions.add_player_hand(trump_chunks[0]);
                frame.render_widget(paragraph, trump_chunks[0]);

                // ジョーカーランク設定エリア
                let joker_rank_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightCyan))
                    .padding(Padding::horizontal(1));

                let joker_rank_label = if app.game.is_player_hand_copy_joker_any(hand_index) {
                    app.game
                        .player_hand()
                        .card(hand_index)
                        .map(|card| format!("> {}", card.name()))
                        .unwrap_or_else(|| "Copy rank to Joker".to_string())
                } else {
                    "Copy rank to Joker".to_string()
                };

                let joker_rank_content = Paragraph::new(joker_rank_label)
                    .block(joker_rank_block);

                app.positions.add_player_hand_copy_joker(trump_chunks[1]);
                frame.render_widget(joker_rank_content, trump_chunks[1]);
            }
        }
    }
}
