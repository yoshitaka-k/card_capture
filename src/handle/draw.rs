use ratatui::layout::Position;

use crate::app::App;
use crate::constants::MAX_HAND_SIZE;
use crate::handle::flags::update_flags;
use crate::handle::shared::clear_select;

/// プレイヤーのデッキからカードを引くイベントを処理する
pub(crate) fn handle_draw(app: &mut App, mouse_pos: Position) {
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
