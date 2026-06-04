use crate::constants::TITLE_END_PHASE_DELAY_TICKS;
use crate::game::Game;
use crate::render::block_position::BlockPosition;

/// 現在の画面を管理する列挙体
#[derive(PartialEq)]
pub enum CurrentScreen {
    Title,
    Main,
    GameClear,
    GameOver,
    Exiting,
}

/// ゲームのフェーズを管理する列挙体
#[derive(PartialEq)]
pub enum GamePhase {
    Title,
    TitleEnd,
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
    /// 残り tick 数。0 になった tick で `advance_phase` する
    pending_phase_advance_ticks: Option<u8>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Title,
            current_phase: GamePhase::Title,
            turn: 1,
            game: Game::new(),
            positions: BlockPosition::default(),
            help_text: String::new(),
            should_quit: false,
            pending_phase_advance_ticks: None,
        }
    }

    pub fn start(&mut self) {
        self.game.start();

        self.game.shuffle_enemy_deck();
        self.game.shuffle_player_deck();

        self.turn = 1;
    }

    pub fn tick(&mut self) {
        if let Some(ticks) = self.pending_phase_advance_ticks.as_mut() {
            *ticks = ticks.saturating_sub(1);
            if *ticks == 0 {
                self.pending_phase_advance_ticks = None;
                self.advance_phase();
            }
        }
    }

    /// 指定 tick 後にフェーズを進める（既に予約済みなら無視）
    pub fn schedule_phase_advance(&mut self, delay_ticks: u8) {
        if self.pending_phase_advance_ticks.is_none() {
            self.pending_phase_advance_ticks = Some(delay_ticks);
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// フェーズを進める
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            GamePhase::Title => GamePhase::TitleEnd,
            GamePhase::TitleEnd => GamePhase::Setup,
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
            GamePhase::TitleEnd => {
                self.schedule_phase_advance(TITLE_END_PHASE_DELAY_TICKS);
            }
            GamePhase::Setup => {
                if self.current_screen == CurrentScreen::Title {
                    self.start();
                    self.current_screen = CurrentScreen::Main;
                }
                self.turn = 1;
            }
            GamePhase::SetupEnd => {
                self.game.initial_end_phase_enemy_deck();
                self.advance_phase();
            }
            GamePhase::Enemy => {
                self.game.compact_enemy_hand();
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
            _ => {}
        }
    }

    pub fn is_discard_phase(&self) -> bool {
        self.current_phase == GamePhase::Discard
    }

    pub fn is_capture_phase(&self) -> bool {
        self.current_phase == GamePhase::Capture
    }

    /// TitleEnd 表示中のローディングスピナー用フレーム（0..4）
    pub fn title_end_spinner_frame(&self) -> usize {
        if self.current_phase != GamePhase::TitleEnd {
            return 0;
        }
        let elapsed = match self.pending_phase_advance_ticks {
            Some(remaining) => TITLE_END_PHASE_DELAY_TICKS.saturating_sub(remaining),
            None => 0,
        };
        elapsed as usize % 4
    }
}
