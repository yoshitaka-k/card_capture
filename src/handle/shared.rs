use crate::app::App;
use crate::constants::MAX_HAND_SIZE;
use crate::game::Game;

/// ビジュアルインデックスを手札インデックスに変換する
#[inline]
pub(crate) fn visual_to_hand_index(visual_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - visual_index
}

/// 敵またはプレイヤーの手札に選択があるか
fn has_any_selection(game: &Game) -> bool {
    game.enemy_select().iter().any(|&s| s)
        || game.player_select().iter().any(|&s| s)
}

/// 選択がなくなったら suit をクリアする
pub(crate) fn clear_suit_if_no_selection(game: &mut Game) {
    if !has_any_selection(game) {
        game.clear_suit();
    }
}

/// 選択状態をクリアする
pub(crate) fn clear_select(app: &mut App) {
    app.game.clear_enemy_select();
    app.game.clear_player_select();
    app.game.clear_suit();
}
