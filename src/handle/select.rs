use ratatui::layout::Position;

use crate::app::App;
use crate::handle::flags::update_flags;
use crate::handle::shared::{clear_player_hand_copy_joker, clear_suit_if_no_selection, visual_to_hand_index};

/// プレイヤーの手札からカードを選択するイベントを処理する
pub(crate) fn player_select_event(app: &mut App, mouse_pos: Position) {
    // プレイヤーの手札からカードを選択する
    for (visual_index, area) in app.positions.player_hand().iter().enumerate() {
        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);

            // 選択トグルを行う
            if app.game.is_player_selected(hand_index) {
                app.game.add_player_select(hand_index, false);
                clear_suit_if_no_selection(&mut app.game);

                if app.game.player_hand().is_joker(hand_index) {
                    clear_player_hand_copy_joker(app);
                }
            } else {
                app.game.select_player_hand(hand_index);
            }

            break;
        }
    }

    // ジョーカーへランクを設定する
    for (visual_index, area) in app.positions.player_hand_copy_joker().iter().enumerate() {
        if !app.game.player_hand().has_joker() {
            app.help_text = format!("No Joker on hand");
            clear_player_hand_copy_joker(app);
            break;
        }

        if area.contains(mouse_pos) {
            let hand_index = visual_to_hand_index(visual_index);

            // カードがあるかどうか
            let has_card = app.game.player_hand().card(hand_index).is_some();
            // ジョーカーが選択されているかどうか
            let is_joker_selected = app.game.is_player_select_joker();
            // 選択されたカードがジョーカーかどうか
            let is_selected_joker = app.game.player_hand().card(hand_index).unwrap().is_joker();


            match (has_card, is_joker_selected, is_selected_joker) {
                // カードがあり、ジョーカーが選択されているが、選択されたカードがジョーカーでない場合
                (true, true, false) => {
                    app.help_text.clear();
                    let selected_joker_ranks = app.game.selected_player_joker_ranks();
                    if selected_joker_ranks.is_empty() {
                        app.help_text = format!("Joker not selected");
                        clear_player_hand_copy_joker(app);
                        break;
                    }

                    let target_joker_rank = selected_joker_ranks
                        .iter()
                        .copied()
                        .find(|&joker_rank| app.game.player_hand_copy_joker_source(joker_rank).is_none())
                        .unwrap_or(selected_joker_ranks[0]);

                    app.game.clear_player_hand_copy_joker(target_joker_rank);
                    app.game.set_player_hand_copy_joker(target_joker_rank, hand_index, true);
                }
                // カードがあり、ジョーカーが選択されていないが、選択されたカードがジョーカーの場合
                (true, false, true) => {
                    app.help_text = format!("Joker not selected");
                    clear_player_hand_copy_joker(app);
                }
                // カードがあり、ジョーカーが選択されていないが、選択されたカードがジョーカーでない場合
                (true, false, false) => {
                    app.help_text = format!("Joker not selected");
                    clear_player_hand_copy_joker(app);
                }
                // 選択されたカードがジョーカーの場合
                (_, _, true) => {
                    app.help_text = format!("This card is a Joker, so it cannot be selected as a copy source");
                    clear_player_hand_copy_joker(app);
                }
                // カードがない場合
                (false, _, _) => {
                    app.help_text = format!("Card not found");
                    clear_player_hand_copy_joker(app);
                }
            }
            break;
        }
    }

    update_flags(app);
}
