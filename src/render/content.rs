use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{app::App};
use crate::render::content_block::{
    enemy::enemy_content,
    middle::middle_content,
    player::player_content,
    phase::phase_content,
    help::help_content,
};

/// Render the content block
pub fn render_content_block(app: &mut App, frame: &mut Frame, area: Rect) {
    app.positions.clear();

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Percentage(35),
            Constraint::Percentage(30),
            Constraint::Percentage(35),
            Constraint::Length(5),
        ])
        .split(area);

    // Phase Content Area Block
    phase_content(app, frame, content_chunks[0]);

    // Enemy Content Area Block
    enemy_content(app, frame, content_chunks[1]);

    // Player Deck & Discard Area, Enemy Discard Area Block
    middle_content(app, frame, content_chunks[2]);

    // Player Hand Area Block
    player_content(app, frame, content_chunks[3]);

    // Help Content Area Block
    help_content(app, frame, content_chunks[4]);
}
