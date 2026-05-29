use ratatui::layout::Position;
use ratatui::crossterm::event::{
    KeyCode,
    KeyEvent,
    KeyModifiers,
    MouseEvent,
    MouseEventKind,
    MouseButton,
};
use crate::constants::{MAX_HAND_SIZE, PHASE_ADVANCE_DELAY_TICKS};
use crate::app::{App, CurrentScreen, GamePhase};

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

    match app.current_phase {
        GamePhase::Setup => {
            handle_enemy_draw(app, mouse_pos);
            if app.game.get_enemy_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        },
        GamePhase::Enemy => {
            handle_enemy_draw(app, mouse_pos);
            if app.game.get_enemy_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        },
        GamePhase::Discard => {
            handle_discard(app, mouse_pos);
        },
        GamePhase::Draw => {
            handle_draw(app, mouse_pos);
            if app.game.get_player_hand().len() == MAX_HAND_SIZE {
                app.schedule_phase_advance(PHASE_ADVANCE_DELAY_TICKS);
            }
        },
        GamePhase::Capture => {
            handle_capture(app, mouse_pos);
        },
        GamePhase::End => handle_end(app),
        _ => {}
    }
}

/// 敵のデッキからカードを引くイベントを処理する
fn handle_enemy_draw(app: &mut App, mouse_pos: Position) {
    // 敵のデッキからカードを引く
    if app.positions.get_enemy_deck().contains(mouse_pos) {
        if app.game.get_enemy_hand().len() < MAX_HAND_SIZE {
            if let Some(card) = app.game.draw_enemy_card() {
                app.game.add_enemy_hand(card);
            }
        }
    }

    clear_select(app);
    update_flags(app);
}

/// 捨て札フェーズのイベントを処理する
fn handle_discard(app: &mut App, mouse_pos: Position) {
    // プレイヤーの手札からカードを選択する
    player_select_event(app, mouse_pos);

    // プレイヤーの選択したカードを捨て札へ送る
    if app.positions.get_player_discard().contains(mouse_pos) {
        if app.game.get_player_select().iter().all(|&selected| !selected) {
            clear_select(app);
            update_flags(app);

            // 捨て札フェーズを終了する
            app.advance_phase();
            return;
        }

        for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
            let hand_index = visual_to_hand_index(visual_index);
            if app.game.is_player_selected(hand_index) {
                if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                    app.game.add_player_discard(player_card.clone());
                    app.game.take_player_hand_card(hand_index);
                }
            }
        }

        clear_select(app);
        update_flags(app);
    }
}

/// 捕獲フェーズのイベントを処理する
fn handle_capture(app: &mut App, mouse_pos: Position) {
    // 敵の手札からカードを選択する
    for (visual_index, area) in app.positions.get_enemy_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);
            if let Some(card) = app.game.get_enemy_hand().get_card(hand_index) {
                if app.game.is_enemy_selected(hand_index) {
                    app.game.clear_enemy_suit();
                    app.game.add_enemy_select(hand_index, false);
                } else {
                    let suit = card.get_suit().clone();
                    app.game.set_enemy_suit(&suit);
                    app.game.add_enemy_select(hand_index, true);
                }
            } else {
                app.game.clear_enemy_suit();
                app.game.add_enemy_select(hand_index, false);
            }

            update_flags(app);

            break;
        }
    }

    // プレイヤーの手札からカードを選択する
    player_select_event(app, mouse_pos);

    // 敵の捨て札クリックイベント処理
    if app.positions.get_enemy_discard().contains(mouse_pos) {
        if app.game.is_enemy_cupture() {
            enemy_capture_event(app);
            app.advance_phase();
        }

        // 生贄処理を行う
        // 選択したプレイヤーカードを敵の捨て札へ
        // 選択した的カードを敵デッキの一番下に追加
        if app.game.is_sacrifice() {
            sacrifice_event(app);
            app.advance_phase();
        }
    }

    // プレイヤーの捨て札へ選択したカードを送る
    if app.positions.get_player_discard().contains(mouse_pos) {
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

/// プレイヤーのデッキからカードを引くイベントを処理する
fn handle_draw(app: &mut App, mouse_pos: Position) {
    // プレイヤーのデッキからカードを引く
    if app.positions.get_player_deck().contains(mouse_pos) {
        if app.game.get_player_hand().len() < MAX_HAND_SIZE {
            if let Some(card) = app.game.draw_player_card() {
                app.game.add_player_hand(card);
            } else {
                let cards = app.game.get_player_discard().clone();
                app.game.set_player_deck_cards(cards);
                app.game.shuffle_player_deck();

                app.game.clear_player_discard();

                if let Some(card) = app.game.draw_player_card() {
                    app.game.add_player_hand(card);
                }
            }
        }
    }

    clear_select(app);
    update_flags(app);
}

/// ゲーム終了フェーズのイベントを処理する
fn handle_end(app: &mut App) {
    clear_select(app);
    update_flags(app);

    app.advance_phase();
}

/// ビジュアルインデックスを手札インデックスに変換する
#[inline]
fn visual_to_hand_index(visual_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - visual_index
}

/// プレイヤーの手札からカードを選択するイベントを処理する
fn player_select_event(app: &mut App, mouse_pos: Position) {
    // プレイヤーの手札からカードを選択する
    for (visual_index, area) in app.positions.get_player_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);
            if let Some(card) = app.game.get_player_hand().get_card(hand_index) {
                if app.game.is_player_selected(hand_index) {
                    app.game.clear_player_suit();
                    app.game.add_player_select(hand_index, false);
                } else {
                    let suit = card.get_suit().clone();
                    app.game.set_player_suit(&suit);
                    app.game.add_player_select(hand_index, true);
                }
            } else {
                app.game.clear_player_suit();
                app.game.add_player_select(hand_index, false);
            }

            update_flags(app);

            break;
        }
    }

    for (visual_index, area) in app.positions.get_player_hand_joker().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);
            if let Some(joker_rank) = app.game.get_player_hand_joker(hand_index) {
                app.help_text = format!("Joker rank: {}", joker_rank);
            } else {
                app.help_text = format!("Select Joker rank");
            }
            break;
        }
    }
}

/// プレイヤーの捕獲イベントを処理する
fn player_capture_event(app: &mut App) {
    // 敵の選択したカードを捨て札へ送る
    for (visual_index, _) in app.positions.get_enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index) {
                app.game.add_player_discard(enemy_card.clone());
                app.game.take_enemy_hand_card(hand_index);
            }
        }
    }

    // プレイヤーの選択したカードを捨て札へ送る
    for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                app.game.add_player_discard(player_card.clone());
                app.game.take_player_hand_card(hand_index);
            }
        }
    }

    clear_select(app);
    update_flags(app);
}

/// 捨て札イベントを処理する
fn discard_event(app: &mut App) {
    for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                app.game.add_player_discard(player_card.clone());
                app.game.take_player_hand_card(hand_index);
            }
        }
    }

    clear_select(app);
    update_flags(app);
}

/// 敵の捕獲イベントを処理する
fn enemy_capture_event(app: &mut App) {
    for (visual_index, _) in app.positions.get_enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index) {
                app.game.add_enemy_discard(enemy_card.clone());
                app.game.take_enemy_hand_card(hand_index);
            }
        }
    }

    // プレイヤーの選択したカードを敵の捨て札へ送る
    for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
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

/// 生贄イベントを処理する
fn sacrifice_event(app: &mut App) {
    // 敵の選択したカードを敵デッキの一番下に追加
    for (visual_index, _) in app.positions.get_enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index) {
                app.game.add_enemy_deck(enemy_card.clone());
                app.game.take_enemy_hand_card(hand_index);
            }
        }
    }

    // プレイヤーの選択したカードを敵の捨て札へ送る
    for (visual_index, _) in app.positions.get_player_hand().iter().enumerate() {
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

/// 選択状態をクリアする
fn clear_select(app: &mut App) {
    app.game.clear_enemy_select();
    app.game.clear_player_select();
}

/// フラグを更新する
fn update_flags(app: &mut App) {
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
    if app.game.get_enemy_select().iter().all(|&selected| !selected)
        || app.game.get_player_select().iter().all(|&selected| !selected) {
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
    if app.game.get_enemy_select().iter().all(|&selected| !selected)
        || app.game.get_player_select().iter().all(|&selected| !selected) {
        app.game.set_enemy_cupture(false);
        return false;
    }

    let enemy_cnt = app.game.get_enemy_select()
        .iter().filter(|&&selected| selected)
        .count();

    let player_cnt = app.game.get_player_select()
        .iter().filter(|&&selected| selected)
        .count();

    if enemy_cnt == 1 && player_cnt == 1 {
        app.game.set_enemy_cupture(true);
        return true;
    }

    app.game.set_enemy_cupture(false);
    false
}

/// 捨て札フラグを更新する
fn update_discard(app: &mut App) -> bool {
    if app.game.get_enemy_select().iter().all(|&selected| selected) {
        app.game.set_discard(false);
        return false;
    }
    if app.game.get_player_select().iter().all(|&selected| !selected) {
        app.game.set_discard(false);
        return false;
    }

    app.game.set_discard(true);
    true
}

/// 生贄フラグを更新する
/// 敵の選択したカードが1枚、プレイヤーの選択したカードが2枚あれば生贄フラグを立てる
fn update_sacrifice(app: &mut App) -> bool {
    if app.game.get_enemy_select().iter().all(|&selected| !selected) {
        app.game.set_sacrifice(false);
        return false;
    }

    let cnt = app.game.get_player_select()
        .iter().filter(|&&selected| selected)
        .count();

    app.game.set_sacrifice(cnt == 2);
    cnt == 2
}
