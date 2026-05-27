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
use crate::trump::Card;

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

    // Player Hand Select
    for (i, area) in app.positions.get_player_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            if let Some(card) = app.game.get_player_hand().get_card(i) {
                app.help_text = format!("Player selected a card: {}", card);
                if app.game.is_player_selected(i) {
                    app.game.add_player_select(i, false);
                } else {
                    app.game.add_player_select(i, true);
                }
            } else {
                app.help_text = String::from("Player selected a card: None");
            }
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

    // Enemy Hand Select
    for (i, area) in app.positions.get_enemy_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            if let Some(card) = app.game.get_enemy_hand().get_card(i) {
                app.help_text = format!("Enemy selected a card: {}", card);
                if let Some(selected_card) = app.game.get_enemy_select() {
                    if card.equals(&selected_card) {
                        app.game.add_enemy_select(None::<Card>);
                    } else {
                        app.game.add_enemy_select(Some(card.clone()));
                    }
                } else {
                    app.game.add_enemy_select(Some(card.clone()));
                }
            } else {
                app.help_text = String::from("Enemy selected a card: None");
            }
            break;
        }
    }
}
