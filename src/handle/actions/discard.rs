use crate::app::App;
use crate::handle::flags::update_flags;
use crate::handle::shared::{clear_select, visual_to_hand_index};

/// 捕獲フェーズでジョーカーのランク表示のみ行うイベントを処理する
pub(crate) fn discard_event(app: &mut App) {
    for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.player_hand().get_card(hand_index) {
                app.help_text = format!("Joker rank: {}", player_card.rank());
            }
        }
    }

    clear_select(app);
    update_flags(app);
}
