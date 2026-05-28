use ratatui::layout::Position;
use ratatui::crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseEvent,
    MouseEventKind,
    MouseButton,
};
use crate::constants::MAX_HAND_SIZE;
use crate::app::{App, CurrentScreen};

/// キーイベントを処理する関数
pub fn key_update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    app.current_screen = CurrentScreen::Exiting;
                }
                _ => {}
            }
        }
        CurrentScreen::Exiting => {
            match key_event.code {
                KeyCode::Enter | KeyCode::Char('y') => {
                    app.should_quit = true;
                    return;
                }
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('q') => {
                    app.current_screen = CurrentScreen::Main;
                    return;
                }
                _ => {}
            }
        }
    }
}

/// マウスイベントを処理する関数
pub fn mouse_update(app: &mut App, mouse_event: MouseEvent) {
    match app.current_screen {
        CurrentScreen::Main => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    handle_mouse_up_left(app, mouse_event);
                }
                _ => {}
            }
        }
        CurrentScreen::Exiting => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Right) => {
                    app.current_screen = CurrentScreen::Main;
                }
                _ => {}
            }
        }
    }
}

/// マウス左クリックイベントを処理する関数
fn handle_mouse_up_left(app: &mut App, mouse_event: MouseEvent) {
    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);

    // 敵のデッキからカードを引く
    if app.positions.get_enemy_deck().contains(mouse_pos)
        && app.game.get_enemy_hand().len() < MAX_HAND_SIZE {
        if let Some(card) = app.game.draw_enemy_card() {
            app.game.add_enemy_hand(card);
        }
    }

    // 敵の手札からカードを選択する
    for (visual_index, area) in app.positions.get_enemy_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);
            if let Some(_) = app.game.get_enemy_hand().get_card(hand_index) {
                if app.game.is_enemy_selected(hand_index) {
                    app.game.add_enemy_select(hand_index, false);
                } else {
                    app.game.add_enemy_select(hand_index, true);
                }
            } else {
                app.game.add_enemy_select(hand_index, false);
            }

            update_sacrifice(app);
            update_player_capture(app);

            break;
        }
    }

    // 敵の捨て札クリックイベント処理
    if app.positions.get_enemy_discard().contains(mouse_pos) {
        // 生贄処理を行う
        // 選択したプレイヤーカードを敵の捨て札へ
        // 選択した的カードを敵デッキの一番下に追加
        if app.game.is_sacrifice() {
            sacrifice_event(app);
        }
    }

    // プレイヤーのデッキからカードを引く
    if app.positions.get_player_deck().contains(mouse_pos)
        && app.game.get_player_hand().len() < MAX_HAND_SIZE {
        if let Some(card) = app.game.draw_player_card() {
            app.game.add_player_hand(card);
        }
    }

    // プレイヤーの捨て札へ選択したカードを送る
    if app.positions.get_player_discard().contains(mouse_pos)
        && app.game.is_player_cupture() {
        // 敵の選択したカードを捨て札へ送る
        for (visual_index, _) in app.positions.get_enemy_hand().iter().enumerate() {
            let hand_index = visual_to_hand_index(visual_index);
            if app.game.is_enemy_selected(hand_index) {
                if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index) {
                    app.game.add_player_discard(enemy_card.clone());
                    app.game.remove_enemy_hand_card(hand_index);
                }
            }
        }

        // プレイヤーの選択したカードを捨て札へ送る
        for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
            let hand_index = visual_to_hand_index(visual_index);
            if app.game.is_player_selected(hand_index) {
                if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                    app.game.add_player_discard(player_card.clone());
                    app.game.remove_player_hand_card(hand_index);
                }
            }
        }

        app.game.clear_enemy_select();
        app.game.clear_player_select();

        update_sacrifice(app);
        update_player_capture(app);
    }

    // プレイヤーの手札からカードを選択する
    for (visual_index, area) in app.positions.get_player_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);
            if let Some(_) = app.game.get_player_hand().get_card(hand_index) {
                if app.game.is_player_selected(hand_index) {
                    app.game.add_player_select(hand_index, false);
                } else {
                    app.game.add_player_select(hand_index, true);
                }
            } else {
                app.game.add_player_select(hand_index, false);
            }

            update_sacrifice(app);
            update_player_capture(app);

            break;
        }
    }
}

/// 生贄イベントを処理する
fn sacrifice_event(app: &mut App) {
    // 敵の選択したカードを敵デッキの一番下に追加
    for (visual_index, _) in app.positions.get_enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index) {
                app.game.add_enemy_deck(enemy_card.clone());
                app.game.remove_enemy_hand_card(hand_index);
            }
        }
    }

    // プレイヤーの選択したカードを敵の捨て札へ送る
    for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                app.game.add_enemy_discard(player_card.clone());
                app.game.remove_player_hand_card(hand_index);
            }
        }
    }

    app.game.clear_enemy_select();
    app.game.clear_player_select();

    update_sacrifice(app);
    update_player_capture(app);
}

/// ビジュアルインデックスを手札インデックスに変換する
#[inline]
fn visual_to_hand_index(visual_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - visual_index
}

/// 捕獲できるか判定する
/// プレイヤーの選択したカードの合計ランクが選択した敵のカードの合計ランクより大きければ捕獲成功
/// それ以外は捕獲失敗
fn update_player_capture(app: &mut App) -> bool {
    if app.game.get_enemy_select().iter().all(|&selected| !selected)
        || app.game.get_player_select().iter().all(|&selected| !selected) {
        app.game.set_player_cupture(false);
        return false;
    }

    let player_select_rank = app.game.calc_player_select_rank();
    let enemy_select_rank = app.game.calc_enemy_select_rank();

    app.help_text = format!("player_select_rank: {}, enemy_select_rank: {}", player_select_rank, enemy_select_rank);

    if player_select_rank >= enemy_select_rank {
        app.game.set_player_cupture(true);
        true
    } else {
        app.game.set_player_cupture(false);
        false
    }
}

/// 生贄フラグを更新する
/// 敵の選択したカードが1枚、プレイヤーの選択したカードが2枚あれば生贄フラグを立てる
fn update_sacrifice(app: &mut App) -> bool {
    if app.game.get_enemy_select().iter().all(|&selected| !selected) {
        app.game.set_sacrifice(false);
        return false;
    }

    let mut sacrifice = 0;
    for selected in app.game.get_player_select().iter() {
        if *selected {
            sacrifice += 1;
        }
    }

    // app.help_text = format!("sacrifice: {}", sacrifice);

    app.game.set_sacrifice(sacrifice == 2);
    sacrifice == 2
}
