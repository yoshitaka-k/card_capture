use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Render the content block
pub fn render_content_block(app: &App, frame: &mut Frame, area: Rect) {
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
    help_content(frame, content_chunks[3]);
}

/// Render the enemy content block
fn enemy_content(app: &App, frame: &mut Frame, area: Rect) {
    // Enemy Block
    let enemy_block = Block::default()
        .title("Enemy Content")
        .title_style(Style::default().fg(Color::Red).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    frame.render_widget(enemy_block, area);

    let enemy_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]).split(area);

    // Deck Block
    let deck_block = Block::default()
        .title("Deck")
        .title_style(Style::default().fg(Color::Red).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let deck_content = Paragraph::new(format!("Enemy Deck count: {}", app.game.enemy_deck.len()))
        .block(deck_block);

    frame.render_widget(deck_content, enemy_chunks[0]);

    // Enemy Hand Block
    let enemy_hand_block = Block::default()
        .title("Enemy Hand")
        .title_style(Style::default().fg(Color::Red).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let enemy_hand_content = Paragraph::new(format!("Enemy Hand count: {}", app.game.enemy_hand.len()))
        .block(enemy_hand_block);

    frame.render_widget(enemy_hand_content, enemy_chunks[1]);
}

/// Render the player content block
fn player_content(app: &App, frame: &mut Frame, area: Rect) {
    let player_block = Block::default()
        .title("Player Content")
        .title_style(Style::default().fg(Color::Blue).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let player_content = Paragraph::new(format!("Player Deck count: {}", app.game.player_deck.len()))
    .block(player_block);

    frame.render_widget(player_content, area);
}

/// Render the hand content block
fn hand_content(app: &App, frame: &mut Frame, area: Rect) {
    let hand_block = Block::default()
        .title("Hand Content")
        .title_style(Style::default().fg(Color::Green).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let hand_content = Paragraph::new(format!("Player Hand count: {}", app.game.player_hand.len()))
        .block(hand_block);

    // let text = format!(
    //     "Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //     Press `k` and `j` to increment and decrement the counter respectively.\n\
    //     Counter: {}
    //   ",
    //     app.counter
    // );
    // let content_text = Text::styled(text, Style::default());

    // let hand_content = Paragraph::new(content_text)
    //     .block(hand_block);

    frame.render_widget(hand_content, area);
}

/// Render the help content block
fn help_content(frame: &mut Frame, area: Rect) {
    let help_block = Block::default()
        .title("Help Content")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let help_content = Paragraph::new("Help Content")
    .block(help_block);

    frame.render_widget(help_content, area);
}
