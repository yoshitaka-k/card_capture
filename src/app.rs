use crate::game::Game;

/// 現在の画面
pub enum CurrentScreen {
    Main,
    Exiting,
}

/// アプリケーション
pub struct App {
    pub current_screen: CurrentScreen,
    pub counter: u8,
    pub game: Game,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            counter: 0,
            game: Game::new(),
            should_quit: false,
        }
    }

    pub fn start(&mut self) {
        self.game.start();
    }

    pub fn tick(&mut self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
