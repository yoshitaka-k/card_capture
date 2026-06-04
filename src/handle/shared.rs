use crate::app::App;
use crate::game::Game;

pub(crate) use crate::hand_index::visual_to_hand_index;

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
    clear_player_hand_copy_joker(app);
}

/// ジョーカーコピー用スロットをクリアする
pub(crate) fn clear_player_hand_copy_joker(app: &mut App) {
    app.game.clear_player_hand_copy_joker(0);
    app.game.clear_player_hand_copy_joker(1);
}
