use ratatui::crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseEvent,
    MouseEventKind,
    MouseButton,
};
use crate::app::{App, CurrentScreen};

pub fn key_update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    app.current_screen = CurrentScreen::Exiting;
                }
                KeyCode::Right | KeyCode::Char('k') => app.increment_counter(),
                KeyCode::Left | KeyCode::Char('j') => app.decrement_counter(),
                _ => {}
            }
        }
        CurrentScreen::Exiting => {
            match key_event.code {
                KeyCode::Enter | KeyCode::Char('y') => {
                    app.should_quit = true;
                    return;
                }
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('q') => {
                    app.current_screen = CurrentScreen::Main;
                    return;
                }
                _ => {}
            }
        }
    }
}

pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    app.current_screen = CurrentScreen::Exiting;
                }
                _ => {}
            }
        }
        CurrentScreen::Exiting => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    app.should_quit = true;
                }
                MouseEventKind::Up(MouseButton::Right) => {
                    app.current_screen = CurrentScreen::Main;
                }
                _ => {}
            }
        }
    }
}
