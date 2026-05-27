use crate::game::Game;
use crate::render::block_position::BlockPosition;

/// 現在の画面を管理する列挙体
pub enum CurrentScreen {
    Main,
    Exiting,
}

/// ゲームのフェーズを管理する列挙体
pub enum GamePhase {
    Initial,
    InitialEnd,
    Enemy,
    Discard,
    Draw,
    Battle,
    End,
}

/// アプリケーション状態を管理する構造体
pub struct App {
    pub current_screen: CurrentScreen,
    pub current_phase: GamePhase,
    pub turn: isize,
    pub game: Game,
    pub positions: BlockPosition,
    pub help_text: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Main,
            current_phase: GamePhase::Initial,
            turn: 0,
            game: Game::new(),
            positions: BlockPosition::default(),
            help_text: String::new(),
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
}
