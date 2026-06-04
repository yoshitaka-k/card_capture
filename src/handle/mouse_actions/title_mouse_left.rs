use crate::app::{App, GamePhase};

/// タイトル画面のマウス左クリックイベントを処理する
pub(crate) fn handle_title_mouse_left(app: &mut App) {
    if app.current_phase == GamePhase::Title {
        app.advance_phase();
    }
}
