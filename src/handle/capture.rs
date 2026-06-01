use ratatui::layout::Position;

use crate::app::App;
use crate::handle::actions::discard::discard_event;
use crate::handle::actions::enemy_capture::enemy_capture_event;
use crate::handle::actions::player_capture::player_capture_event;
use crate::handle::actions::sacrifice::sacrifice_event;
use crate::handle::flags::update_flags;
use crate::handle::select::player_select_event;
use crate::handle::shared::{clear_suit_if_no_selection, visual_to_hand_index};

/// 捕獲フェーズのイベントを処理する
pub(crate) fn handle_capture(app: &mut App, mouse_pos: Position) {
    for (visual_index, area) in app.positions.enemy_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);

            if app.game.is_enemy_selected(hand_index) {
                app.game.add_enemy_select(hand_index, false);
                clear_suit_if_no_selection(&mut app.game);
            } else if let Some(card) = app.game.enemy_hand().get_card(hand_index) {
                let suit = card.get_suit().clone();
                app.game.add_enemy_select(hand_index, true);

                if app.game.player_select().iter().all(|&s| !s) {
                    app.game.clear_suit();
                    app.game.set_suit(&suit);
                }
            }

            update_flags(app);

            break;
        }
    }

    player_select_event(app, mouse_pos);

    if app.positions.enemy_discard().contains(mouse_pos) {
        if app.game.is_enemy_cupture() {
            enemy_capture_event(app);
            app.advance_phase();
        }

        if app.game.is_sacrifice() {
            sacrifice_event(app);
            app.advance_phase();
        }
    }

    if app.positions.player_discard().contains(mouse_pos) {
        if app.game.is_player_cupture() {
            player_capture_event(app);
            app.advance_phase();
        }

        if app.game.is_discard() {
            discard_event(app);
            app.advance_phase();
        }
    }
}
