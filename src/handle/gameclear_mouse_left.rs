use crate::app::{App, CurrentScreen, GamePhase};

/// ゲームクリア画面のマウス左クリックイベントを処理する
pub(crate) fn handle_gameclear_mouse_left(app: &mut App) {
    match app.current_phase {
        GamePhase::End => {
            app.start();

            app.current_screen = CurrentScreen::Main;
            app.current_phase = GamePhase::Setup;
        }
        _ => {}
    }
}
