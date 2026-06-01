use ratatui::layout::Position;
use ratatui::crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseEvent,
    MouseEventKind,
    MouseButton,
};
use crate::constants::{MAX_HAND_SIZE, PHASE_ADVANCE_DELAY_TICKS};
use crate::app::{App, CurrentScreen, GamePhase};
use crate::handle::capture::handle_capture;
use crate::handle::discard::handle_discard;
use crate::handle::draw::handle_draw;
use crate::handle::end::handle_end;
use crate::handle::enemy_draw::handle_enemy_draw;
use crate::handle::gameover_mouse_left::handle_gameover_mouse_left;

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
        CurrentScreen::GameClear => {}
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

/// マウス左クリックイベントを処理する関数
fn handle_main_mouse_left(app: &mut App, mouse_event: MouseEvent) {
    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

    match app.current_phase {
        GamePhase::Setup => {
            handle_enemy_draw(app, mouse_pos);
            if app.game.get_enemy_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        },
        GamePhase::Enemy => {
            handle_enemy_draw(app, mouse_pos);
            if app.game.get_enemy_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        },
        GamePhase::Discard => {
            handle_discard(app, mouse_pos);
        },
        GamePhase::Draw => {
            handle_draw(app, mouse_pos);
            if app.game.get_player_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        },
        GamePhase::Capture => {
            handle_capture(app, mouse_pos);
        },
        GamePhase::End => handle_end(app),
        _ => {}
    }
}
