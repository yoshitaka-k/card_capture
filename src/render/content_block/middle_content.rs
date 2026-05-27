use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Render the player content block
pub fn middle_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let trash_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ]).split(area);

    // Player Discard Block
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

    // Discard Block
    let enemy_trash_block = Block::default()
        .title("Discard")
        .title_style(Style::default().fg(Color::DarkGray).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .padding(Padding::horizontal(1));

    let enemy_trash_content = Paragraph::new(
        format!("Discard count: {}", app.game.get_enemy_discard().len())
    ).block(enemy_trash_block);

    app.positions.set_enemy_discard(trash_chunks[2]);
    frame.render_widget(enemy_trash_content, trash_chunks[2]);
}
