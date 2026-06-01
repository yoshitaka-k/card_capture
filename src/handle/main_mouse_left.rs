use ratatui::crossterm::event::MouseEvent;
use ratatui::layout::Position;

use crate::app::{App, GamePhase};
use crate::constants::{MAX_HAND_SIZE, PHASE_ADVANCE_DELAY_TICKS};
use crate::handle::capture::handle_capture;
use crate::handle::discard::handle_discard;
use crate::handle::draw::handle_draw;
use crate::handle::end::handle_end;
use crate::handle::enemy_draw::handle_enemy_draw;

/// メイン画面のマウス左クリックイベントを処理する
pub(crate) fn handle_main_mouse_left(app: &mut App, mouse_event: MouseEvent) {
    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

    match app.current_phase {
        GamePhase::Setup => {
            handle_enemy_draw(app, mouse_pos);
            if app.game.get_enemy_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        }
        GamePhase::Enemy => {
            handle_enemy_draw(app, mouse_pos);
            if app.game.get_enemy_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        }
        GamePhase::Discard => {
            handle_discard(app, mouse_pos);
        }
        GamePhase::Draw => {
            handle_draw(app, mouse_pos);
            if app.game.get_player_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        }
        GamePhase::Capture => {
            handle_capture(app, mouse_pos);
        }
        GamePhase::End => handle_end(app),
        _ => {}
    }
}
