use ratatui::layout::Position;

use crate::app::App;
use crate::constants::MAX_HAND_SIZE;
use crate::handle::flags::update_flags;
use crate::handle::shared::clear_select;

/// 敵のデッキからカードを引くイベントを処理する
pub(crate) fn handle_enemy_draw(app: &mut App, mouse_pos: Position) {
    if app.positions.enemy_deck().contains(mouse_pos) {
        if app.game.enemy_hand().len() < MAX_HAND_SIZE {
            if let Some(card) = app.game.draw_enemy_card() {
                app.game.add_enemy_hand(card);
            }
        }
    }

    clear_select(app);
    update_flags(app);
}
