pub const MAX_HAND_SIZE: usize = 4;

pub const TICK_RATE_MILLIS: u64 = 200;

/// 敵手札が揃ったあと、次フェーズへ進むまでの tick 数（1 tick = TICK_RATE_MILLIS）
pub const PHASE_ADVANCE_DELAY_TICKS: u8 = 8;

/// タイトル終了フェーズの表示時間（1 tick = TICK_RATE_MILLIS）
pub const TITLE_END_PHASE_DELAY_TICKS: u8 = 20;
