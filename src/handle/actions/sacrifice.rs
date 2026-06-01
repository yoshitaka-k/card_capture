use crate::app::App;
use crate::handle::flags::update_flags;
use crate::handle::shared::{clear_select, visual_to_hand_index};

/// 生贄イベントを処理する
/// 敵の選択したカードを敵デッキの一番下に追加
/// プレイヤーの選択したカードを敵の捨て札へ送る
/// 敵のデッキに戻したカードがA、K、Q、Jの場合はゲームオーバー
pub(crate) fn sacrifice_event(app: &mut App) {
    for (visual_index, _) in app.positions.enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index).cloned() {
                let gameover = enemy_card.is_ace_card() || enemy_card.is_face_card();

                app.game.add_enemy_deck(enemy_card);
                app.game.take_enemy_hand_card(hand_index);
                app.game.set_gameover(gameover);
            }
        }
    }

    for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                app.game.add_enemy_discard(player_card.clone());
                app.game.take_player_hand_card(hand_index);
            }
        }
    }

    clear_select(app);
    update_flags(app);
}
