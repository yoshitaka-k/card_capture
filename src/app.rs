use crate::game::Game;
use crate::render::block_position::BlockPosition;

/// 現在の画面を管理する列挙体
pub enum CurrentScreen {
    Main,
    Exiting,
}

/// ゲームのフェーズを管理する列挙体
#[derive(PartialEq)]
pub enum GamePhase {
    Setup,
    SetupEnd,
    Enemy,
    Discard,
    Draw,
    Capture,
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
            current_phase: GamePhase::Setup,
            turn: 1,
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

    /// フェーズを進める
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GamePhase::Setup => GamePhase::SetupEnd,
            GamePhase::SetupEnd => GamePhase::Enemy,
            GamePhase::Enemy => GamePhase::Discard,
            GamePhase::Discard => GamePhase::Draw,
            GamePhase::Draw => GamePhase::Capture,
            GamePhase::Capture => GamePhase::End,
            GamePhase::End => {
                self.turn += 1;
                GamePhase::Enemy  // 2 ターン目以降は敵から
            }
        };
        self.on_phase_enter();
    }

    /// フェーズ入り時の初期化（選択クリア、フラグリセットなど）
    fn on_phase_enter(&mut self) {
        // フェーズ入り時の初期化（選択クリア、フラグリセットなど）
        match self.current_phase {
            GamePhase::Setup => {
                self.turn = 1;
            }
            GamePhase::SetupEnd => {
                self.game.initial_end_phase_enemy_deck();
                self.advance_phase();
            }
            GamePhase::Enemy => {
                self.game.compact_enemy_hand();
            }
            GamePhase::Discard => {
            }
            GamePhase::Draw => {
                self.game.compact_player_hand();
            }
            GamePhase::Capture => {
                self.game.clear_enemy_select();
                self.game.clear_player_select();
                self.game.set_player_cupture(false);
                self.game.set_enemy_cupture(false);
                self.game.set_discard(false);
                self.game.set_sacrifice(false);
            }
            GamePhase::End => {
            }
        }
    }

    pub fn is_initial_phase(&self) -> bool {
        self.current_phase == GamePhase::Setup
    }

    pub fn is_initial_end_phase(&self) -> bool {
        self.current_phase == GamePhase::SetupEnd
    }

    pub fn is_enemy_phase(&self) -> bool {
        self.current_phase == GamePhase::Enemy
    }

    pub fn is_discard_phase(&self) -> bool {
        self.current_phase == GamePhase::Discard
    }

    pub fn is_draw_phase(&self) -> bool {
        self.current_phase == GamePhase::Draw
    }

    pub fn is_capture_phase(&self) -> bool {
        self.current_phase == GamePhase::Capture
    }

    pub fn is_end_phase(&self) -> bool {
        self.current_phase == GamePhase::End
    }
}
