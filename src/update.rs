use ratatui::crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseEvent,
    MouseEventKind,
    MouseButton,
};
use crate::app::{App, CurrentScreen};
use crate::handle::gameclear_mouse_left::handle_gameclear_mouse_left;
use crate::handle::gameover_mouse_left::handle_gameover_mouse_left;
use crate::handle::main_mouse_left::handle_main_mouse_left;

/// キーイベントを処理する関数
pub fn key_update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Exiting => {
            match key_event.code {
                KeyCode::Enter | KeyCode::Char('y') => {
                    app.should_quit = true;
                    return;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    app.current_screen = CurrentScreen::Main;
                    return;
                }
                _ => {}
            }
        }
        _ => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    app.current_screen = CurrentScreen::Exiting;
                }
                _ => {}
            }
        }
    }
}

/// マウスイベントを処理する関数
pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    handle_main_mouse_left(app, mouse_event);
                }
                _ => {}
            }
        }
        CurrentScreen::GameClear => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    handle_gameclear_mouse_left(app);
                }
                _ => {}
            }
        }
        CurrentScreen::GameOver => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    handle_gameover_mouse_left(app);
                }
                _ => {}
            }
        }
        CurrentScreen::Exiting => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Right) => {
                    app.current_screen = CurrentScreen::Main;
                }
                _ => {}
            }
        }
    }
}
