use crate::app::{App, CurrentScreen};
use crate::handle::flags::update_flags;
use crate::handle::shared::clear_select;
use crate::constants::MAX_HAND_SIZE;

/// エンドフェーズの処理を行う
pub(crate) fn handle_end(app: &mut App) {
    clear_select(app);
    update_flags(app);

    // ゲームクリア判定
    // CardSet::len() は None スロットを含むため、実カードが存在するかを判定
    let is_enemy_hand_empty = (0..MAX_HAND_SIZE)
        .all(|index| app.game.enemy_hand().card(index).is_none());

    //敵の山札が空で、敵の手札が空の場合はゲームクリア
    if app.game.enemy_deck().len() == 0 && is_enemy_hand_empty {
        app.current_screen = CurrentScreen::GameClear;
        return;
    }

    // ゲームオーバーの場合はゲームオーバー画面に遷移
    if app.game.is_gameover() {
        app.current_screen = CurrentScreen::GameOver;
        return;
    }

    app.advance_phase();
}
