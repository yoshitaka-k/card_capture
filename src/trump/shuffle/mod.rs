pub mod double_cut;
pub mod hindu;
pub mod riffle;
pub mod deal;

pub use double_cut::double_cut;
pub use hindu::{hindu_shuffle, HinduParams};
pub use riffle::{riffle_shuffle, RiffleParams};
pub use deal::{deal_shuffle, DealParams};

use std::ops::{Bound, RangeBounds};
use std::time::{SystemTime, UNIX_EPOCH};

/// 現在の時刻をシード値にして乱数生成器を初期化
pub fn seed() -> u64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration.as_micros() as u64
}

/// 簡易な乱数生成器
struct SimpleRand {
    seed: u64,
}
impl SimpleRand {
    pub fn new() -> Self {
        SimpleRand { seed: seed() }
    }

    // 次の乱数を生成する（LCGの超有名な定数だよっ）
    fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.seed
    }

    // 指定範囲の乱数を返す（ダイスや手札の選択にピッタリ！）
    pub fn next_range<R>(&mut self, range: R) -> isize
    where
        R: RangeBounds<isize>,
    {
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => isize::MIN,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n - 1,
            Bound::Unbounded => isize::MAX,
        };
        let span = (end - start + 1) as u64;
        start + (self.next() % span) as isize
    }
}

/// 真ん中あたりの位置を取得（少しだけランダム）
pub fn center_position(cards_len: usize) -> usize {
    if cards_len == 0 {
        return 0;
    }

    let mut rng = SimpleRand::new();
    let base = cards_len / 2;
    let jitter = (cards_len / 10).max(1);
    (base as isize + rng.next_range(-(jitter as isize)..=jitter as isize))
        .clamp(0, cards_len as isize - 1) as usize
}
