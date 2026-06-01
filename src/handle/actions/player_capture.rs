use crate::app::App;
use crate::handle::flags::update_flags;
use crate::handle::shared::{clear_select, visual_to_hand_index};

/// プレイヤーの捕獲イベントを処理する
pub(crate) fn player_capture_event(app: &mut App) {
    for (visual_index, _) in app.positions.enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.enemy_hand().get_card(hand_index) {
                app.game.add_player_discard(enemy_card.clone());
                app.game.take_enemy_hand_card(hand_index);
            }
        }
    }

    for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.player_hand().get_card(hand_index) {
                app.game.add_player_discard(player_card.clone());
                app.game.take_player_hand_card(hand_index);
            }
        }
    }

    clear_select(app);
    update_flags(app);
}
