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
use crate::handle::flags::update_flags;
use crate::handle::shared::{clear_select, clear_suit_if_no_selection, visual_to_hand_index};

/// キーイベントを処理する関数
pub fn key_update(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        CurrentScreen::Exiting => {
            match key_event.code {
                KeyCode::Enter | KeyCode::Char('y') => {
                    app.should_quit = true;
                    return;
                }
                KeyCode::Esc | KeyCode::Char('n') => {
                    app.current_screen = CurrentScreen::Main;
                    return;
                }
                _ => {}
            }
        }
        _ => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => app.current_screen = CurrentScreen::Exiting,
                KeyCode::Char('c') | KeyCode::Char('C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    app.current_screen = CurrentScreen::Exiting;
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
                    handle_main_mouse_left(app, mouse_event);
                }
                _ => {}
            }
        }
        CurrentScreen::GameClear => {}
        CurrentScreen::GameOver => {
            match mouse_event.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    handle_gameover_mouse_left(app);
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
fn handle_main_mouse_left(app: &mut App, mouse_event: MouseEvent) {
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

/// ゲームオーバー画面のマウス左クリックイベントを処理する
fn handle_gameover_mouse_left(app: &mut App) {
    match app.current_phase {
        GamePhase::End => {
            app.start();

            app.current_phase = GamePhase::Setup;
            app.current_screen = CurrentScreen::Main;
        }
        _ => {}
    }
}

/// 敵のデッキからカードを引くイベントを処理する
fn handle_enemy_draw(app: &mut App, mouse_pos: Position) {
    // 敵のデッキからカードを引く
    if app.positions.enemy_deck().contains(mouse_pos) {
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
    if app.positions.player_discard().contains(mouse_pos) {
        // 未選択の場合は捨て札フェーズを終了する
        if app.game.get_player_select().iter().all(|&selected| !selected) {
            clear_select(app);
            update_flags(app);

            // 捨て札フェーズを終了する
            app.advance_phase();
            return;
        }

        // 選択したカードを捨て札へ送る
        for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
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
    for (visual_index, area) in app.positions.enemy_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);

            // 選択トグルを行う
            if app.game.is_enemy_selected(hand_index) {
                app.game.add_enemy_select(hand_index, false);
                clear_suit_if_no_selection(&mut app.game);
            } else if let Some(card) = app.game.get_enemy_hand().get_card(hand_index) {
                let suit = card.get_suit().clone();
                app.game.add_enemy_select(hand_index, true);

                // 敵手札は1枚のみ選択可のため、切り替え時もスートを更新する
                if app.game.get_player_select().iter().all(|&s| !s) {
                    app.game.clear_suit();
                    app.game.set_suit(&suit);
                }
            }

            update_flags(app);

            break;
        }
    }

    // プレイヤーの手札からカードを選択する
    player_select_event(app, mouse_pos);

    // 敵の捨て札クリックイベント処理
    if app.positions.enemy_discard().contains(mouse_pos) {
        // 敵の捕獲処理を行う
        // 敵カードの右端1枚と、プレイヤーの選択したカード1枚を敵の捨て札へ送る
        if app.game.is_enemy_cupture() {
            enemy_capture_event(app);
            app.advance_phase();
        }

        // 生贄処理を行う
        // 敵カードの右端1枚と、プレイヤーの選択したカード2枚で、
        // 敵カードを敵山札の一番下へ、プレイヤーカードを敵の捨て札へ送る
        if app.game.is_sacrifice() {
            sacrifice_event(app);
            app.advance_phase();
        }
    }

    // プレイヤーの捨て札へ選択したカードを送る
    if app.positions.player_discard().contains(mouse_pos) {
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
    if app.positions.player_deck().contains(mouse_pos) {
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

/// エンドフェーズの処理を行う
fn handle_end(app: &mut App) {
    clear_select(app);
    update_flags(app);

    // 捨て札にA、K、Q、Jがある場合はゲームオーバー
    if app.game.is_gameover() {
        app.current_screen = CurrentScreen::GameOver;
    } else {
        app.advance_phase();
    }
}

/// プレイヤーの手札からカードを選択するイベントを処理する
fn player_select_event(app: &mut App, mouse_pos: Position) {
    // プレイヤーの手札からカードを選択する
    for (visual_index, area) in app.positions.player_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);

            // 選択トグルを行う
            if app.game.is_player_selected(hand_index) {
                app.game.add_player_select(hand_index, false);
                clear_suit_if_no_selection(&mut app.game);

                if app.game.get_player_hand().is_joker(hand_index) {
                    app.game.clear_player_hand_copy_joker(0);
                }
            } else if let Some(card) = app.game.get_player_hand().get_card(hand_index) {
                let suit = card.get_suit().clone();
                app.game.add_player_select(hand_index, true);
                app.game.set_suit(&suit);
            }

            break;
        }
    }

    // ジョーカーへランクを設定する
    for (visual_index, area) in app.positions.player_hand_copy_joker().iter().enumerate() {
        if !app.game.get_player_hand().has_joker() {
            app.help_text = format!("No Joker on hand");
            app.game.clear_player_hand_copy_joker(0);
            break;
        }

        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);

            // カードがあるかどうか
            let has_card = app.game.get_player_hand().get_card(hand_index).is_some();
            // ジョーカーが選択されているかどうか
            let is_joker_selected = app.game.is_player_select_joker();
            // 選択されたカードがジョーカーかどうか
            let is_selected_joker = app.game.get_player_hand().get_card(hand_index).unwrap().is_joker();

            app.game.clear_player_hand_copy_joker(0);

            match (has_card, is_joker_selected, is_selected_joker) {
                // カードがあり、ジョーカーが選択されているが、選択されたカードがジョーカーでない場合
                (true, true, false) => {
                    app.help_text.clear();
                    app.game.set_player_hand_copy_joker(0, hand_index, true);
                }
                // カードがあり、ジョーカーが選択されていないが、選択されたカードがジョーカーの場合
                (true, false, true) => {
                    app.help_text = format!("Joker not selected");
                }
                // カードがあり、ジョーカーが選択されていないが、選択されたカードがジョーカーでない場合
                (true, false, false) => {
                    app.help_text = format!("Joker not selected");
                }
                // 選択されたカードがジョーカーの場合
                (_, _, true) => {
                    app.help_text = format!("This card is a Joker, so it cannot be selected as a copy source");
                }
                // カードがない場合
                (false, _, _) => {
                    app.help_text = format!("Card not found");
                }
            }
            break;
        }
    }

    update_flags(app);
}

/// プレイヤーの捕獲イベントを処理する
fn player_capture_event(app: &mut App) {
    // 敵の選択したカードを捨て札へ送る
    for (visual_index, _) in app.positions.enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index) {
                app.game.add_player_discard(enemy_card.clone());
                app.game.take_enemy_hand_card(hand_index);
            }
        }
    }

    // プレイヤーの選択したカードを捨て札へ送る
    for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
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
    for (visual_index, _) in app.positions.player_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_player_selected(hand_index) {
            if let Some(player_card) = app.game.get_player_hand().get_card(hand_index) {
                app.help_text = format!("Joker rank: {}", player_card.get_rank());
            }
        }
    }

    clear_select(app);
    update_flags(app);
}

/// 敵の捕獲イベントを処理する
/// 敵の選択したカードと、プレイヤーの選択したカードを敵の捨て札へ送る
/// 敵の捨て札に追加した敵のカードがA、K、Q、Jの場合はゲームオーバー
fn enemy_capture_event(app: &mut App) {
    // 敵の選択したカードを敵の捨て札へ送る
    for (visual_index, _) in app.positions.enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index).cloned() {
                // 敵の捨て札に追加したカードがA、K、Q、Jの場合はゲームオーバー
                let gameover = enemy_card.is_ace_card() || enemy_card.is_face_card();

                app.game.add_enemy_discard(enemy_card);
                app.game.take_enemy_hand_card(hand_index);
                app.game.set_gameover(gameover);
            }
        }
    }

    // プレイヤーの選択したカードを敵の捨て札へ送る
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

/// 生贄イベントを処理する
/// 敵の選択したカードを敵デッキの一番下に追加
/// プレイヤーの選択したカードを敵の捨て札へ送る
/// 敵のデッキに戻したカードがA、K、Q、Jの場合はゲームオーバー
fn sacrifice_event(app: &mut App) {
    // 敵の選択したカードを敵デッキの一番下に追加
    for (visual_index, _) in app.positions.enemy_hand().iter().enumerate() {
        let hand_index = visual_to_hand_index(visual_index);
        if app.game.is_enemy_selected(hand_index) {
            if let Some(enemy_card) = app.game.get_enemy_hand().get_card(hand_index).cloned() {
                // 敵のデッキに戻したカードがA、K、Q、Jの場合はゲームオーバー
                let gameover = enemy_card.is_ace_card() || enemy_card.is_face_card();

                app.game.add_enemy_deck(enemy_card);
                app.game.take_enemy_hand_card(hand_index);
                app.game.set_gameover(gameover);
            }
        }
    }

    // プレイヤーの選択したカードを敵の捨て札へ送る
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
