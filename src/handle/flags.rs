use crate::app::App;

/// フラグを更新する
pub(crate) fn update_flags(app: &mut App) {
    if app.is_discard_phase() {
        update_discard(app);
    }

    if app.is_capture_phase() {
        update_enemy_capture(app);
        update_player_capture(app);
        update_sacrifice(app);
    }
}

/// 捕獲できるか判定する
/// プレイヤーの選択したカードの合計ランクが選択した敵のカードの合計ランクより大きければ捕獲成功
/// それ以外は捕獲失敗
fn update_player_capture(app: &mut App) -> bool {
    if app.game.enemy_select().iter().all(|&selected| !selected)
        || app.game.player_select().iter().all(|&selected| !selected) {
        app.game.set_player_cupture(false);
        return false;
    }

    // ジョーカーが選択されている場合、コピー元が選択されていなければ捕獲失敗
    if app.game.is_player_select_joker()
        && !app.game.is_selected_player_joker_copy_ready()
    {
        app.game.set_player_cupture(false);
        return false;
    }

    let player_select_rank = app.game.calc_player_select_rank();
    let enemy_select_rank = app.game.calc_enemy_select_rank();

    if player_select_rank >= enemy_select_rank {
        app.game.set_player_cupture(true);
        true
    } else {
        app.game.set_player_cupture(false);
        false
    }
}

/// 敵の捕獲フラグを更新する
/// 敵の手札の右端選択と、プレイヤーの手札の1枚選択があれば敵の捕獲フラグを立てる
fn update_enemy_capture(app: &mut App) -> bool {
    if app.game.enemy_select().iter().all(|&selected| !selected)
        || app.game.player_select().iter().all(|&selected| !selected) {
        app.game.set_enemy_cupture(false);
        return false;
    }

    let enemy_cnt = app.game.enemy_select()
        .iter().filter(|&&selected| selected)
        .count();

    let player_cnt = app.game.player_select()
        .iter().filter(|&&selected| selected)
        .count();

    // 1枚ずつ選択されてる
    if enemy_cnt == 1 && player_cnt == 1 {
        // 敵カードの0番目が選択されているか確認
        if app.game.is_enemy_selected(0) {
            app.game.set_enemy_cupture(true);
            return true;
        }

        app.game.set_enemy_cupture(false);
        return false;
    }

    app.game.set_enemy_cupture(false);
    false
}

/// 捨て札フラグを更新する
fn update_discard(app: &mut App) -> bool {
    if app.game.enemy_select().iter().all(|&selected| selected) {
        app.game.set_discard(false);
        return false;
    }
    if app.game.player_select().iter().all(|&selected| !selected) {
        app.game.set_discard(false);
        return false;
    }

    app.game.set_discard(true);
    true
}

/// 生贄フラグを更新する
/// 敵の選択したカードが1枚、プレイヤーの選択したカードが2枚あれば生贄フラグを立てる
fn update_sacrifice(app: &mut App) -> bool {
    if app.game.enemy_select().iter().all(|&selected| !selected) {
        app.game.set_sacrifice(false);
        return false;
    }

    let cnt = app.game.player_select()
        .iter().filter(|&&selected| selected)
        .count();

    app.game.set_sacrifice(cnt == 2);
    cnt == 2
}
