use ratatui::layout::Position;

use crate::app::App;
use crate::handle::flags::update_flags;
use crate::handle::select::player_select_event;
use crate::handle::shared::{clear_select, visual_to_hand_index};

/// 捨て札フェーズのイベントを処理する
pub(crate) fn handle_discard(app: &mut App, mouse_pos: Position) {
    player_select_event(app, mouse_pos);

    if app.positions.player_discard().contains(mouse_pos) {
        if app.game.player_select().iter().all(|&selected| !selected) {
            clear_select(app);
            update_flags(app);

            app.advance_phase();
            return;
        }

        for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
            let hand_index = visual_to_hand_index(visual_index);
            if app.game.is_player_selected(hand_index) {
                if let Some(player_card) = app.game.take_player_hand_card(hand_index) {
                    app.game.add_player_discard(player_card);
                }
            }
        }

        clear_select(app);
        update_flags(app);
    }
}
