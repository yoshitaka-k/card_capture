/// Render the UI components
pub mod render;
/// Application
pub mod app;
/// Event
pub mod event;
/// User interface
pub mod ui;
/// TUI
pub mod tui;
/// Application updater
pub mod update;
/// Trump
pub mod trump;
/// Game
pub mod game;

use ratatui::layout::{
    Constraint,
    Direction,
    Layout,
    Rect,
};

/// Helper function to center a rectangle
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
