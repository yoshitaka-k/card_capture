/// Constants
pub mod constants;
/// Hand slot index conversion (visual layout vs storage)
pub mod hand_index;
/// Application
pub mod app;
/// Event
pub mod event;
/// User interface
pub mod ui;
/// TUI
pub mod tui;
/// Render the UI components
pub mod render;
/// Application updater
pub mod update;
/// Game input handlers
pub mod handle;
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

/// 表示させた後少し待機
pub fn wait_for_dramatic_pause() {
    std::thread::sleep(std::time::Duration::from_millis(200));
}

/// 表示させた後少し待機
pub fn wait_for_long_dramatic_pause() {
    std::thread::sleep(std::time::Duration::from_millis(3000));
}

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
