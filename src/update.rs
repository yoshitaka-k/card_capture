use ratatui::layout::Position;
use ratatui::crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseEvent,
    MouseEventKind,
    MouseButton,
};
use crate::constants::MAX_HAND_SIZE;
use crate::app::{App, CurrentScreen};

/// キーイベントを処理する関数
pub fn key_update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    app.current_screen = CurrentScreen::Exiting;
                }
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

/// マウスイベントを処理する関数
pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
                    if app.positions.get_player_deck().contains(mouse_pos) {
                        if app.game.get_player_hand().len() < MAX_HAND_SIZE {
                            if let Some(card) = app.game.draw_player_card() {
                                app.game.add_player_hand(card);
                            }
                        }
                    }
                    if app.positions.get_enemy_deck().contains(mouse_pos) {
                        if app.game.get_enemy_hand().len() < MAX_HAND_SIZE {
                            if let Some(card) = app.game.draw_enemy_card() {
                                app.game.add_enemy_hand(card);
                            }
                        }
                    }
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
