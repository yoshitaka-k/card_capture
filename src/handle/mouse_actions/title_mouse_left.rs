use crate::app::{App, CurrentScreen, GamePhase};

/// タイトル画面のマウス左クリックイベントを処理する
pub(crate) fn handle_title_mouse_left(app: &mut App) {
    match app.current_phase {
        GamePhase::Title => {
            app.start();

            app.current_screen = CurrentScreen::Main;
            app.current_phase = GamePhase::Setup;
        }
        _ => {}
    }
}
