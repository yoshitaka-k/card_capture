use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::render::{
    header::render_header_block,
    content::render_content_block,
    footer::render_footer_block,
    exiting::render_exit_popup_block,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title Block
    render_header_block(frame, chunks[0]);

    if let CurrentScreen::Main = app.current_screen {
        // Content Block
        render_content_block(app, frame, chunks[1]);
    }

    // Footer Block
    render_footer_block(app, frame, chunks[2]);

    // exit popup window
    if let CurrentScreen::Exiting = app.current_screen {
        render_exit_popup_block(frame);
    }
}
