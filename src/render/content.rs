use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{app::App};
use crate::render::content_block::{
    enemy_content::enemy_content,
    middle_content::middle_content,
    player_content::player_content,
    help_content::help_content,
};

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

    // Enemy Content Area Block
    enemy_content(app, frame, content_chunks[0]);

    // Player Deck & Discard Area, Enemy Discard Area Block
    middle_content(app, frame, content_chunks[1]);

    // Player Hand Area Block
    player_content(app, frame, content_chunks[2]);

    // Help Content Area Block
    help_content(app, frame, content_chunks[3]);
}
