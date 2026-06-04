use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::render::{
    header::render_header_block,
    content::render_content_block,
    title::render_title_block,
    gameover::render_gameover_block,
    gameclear::render_gameclear_block,
    footer::render_footer_block,
    popup::exiting::render_exit_popup_block,
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

    // popup window
    match app.current_screen {
        CurrentScreen::Title => {
            render_title_block(app, frame, chunks[1]);
        }
        CurrentScreen::Main => {
            render_content_block(app, frame, chunks[1]);
        }
        CurrentScreen::GameClear => {
            render_gameclear_block(app, frame, chunks[1]);
        }
        CurrentScreen::GameOver => {
            render_gameover_block(app, frame, chunks[1]);
        }
        CurrentScreen::Exiting => {
            render_exit_popup_block(frame);
        }
    }

    // Footer Block
    render_footer_block(app, frame, chunks[2]);
}
