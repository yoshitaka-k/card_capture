use crate::app::{App, CurrentScreen};
use crate::handle::flags::update_flags;
use crate::handle::shared::clear_select;

/// エンドフェーズの処理を行う
pub(crate) fn handle_end(app: &mut App) {
    clear_select(app);
    update_flags(app);

    if app.game.is_gameover() {
        app.current_screen = CurrentScreen::GameOver;
    } else {
        app.advance_phase();
    }
}
