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

fn handle_mouse_up_left(app: &mut App, mouse_event: MouseEvent) {
    let mouse_pos = Position::new(mouse_event.column, mouse_event.row);
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
            update_player_capture(app);
            break;
        }
    }

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
            update_player_capture(app);
            break;
        }
    }
}

#[inline]
fn visual_to_hand_index(visual_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - visual_index
}

/// 捕獲できるか判定する
fn update_player_capture(app: &mut App) -> bool {
    if app.game.get_enemy_select().iter().all(|&selected| !selected)
        || app.game.get_player_select().iter().all(|&selected| !selected) {
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
