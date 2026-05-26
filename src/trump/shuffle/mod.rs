pub mod double_cut;
pub mod hindu;
pub mod riffle;
pub mod deal;

pub use double_cut::double_cut;
pub use hindu::{hindu_shuffle, HinduParams};
pub use riffle::{riffle_shuffle, RiffleParams};
pub use deal::{deal_shuffle, DealParams};

use rand::RngExt;

/// 真ん中あたりの位置を取得（少しだけランダム）
pub fn get_center_position(cards_len: usize) -> usize {
    if cards_len == 0 {
        return 0;
    }
    let mut rng = rand::rng();
    let base = cards_len / 2;
    let jitter = (cards_len / 10).max(1);
    (base as isize + rng.random_range(-(jitter as i64)..=(jitter as i64)) as isize)
        .clamp(0, cards_len as isize - 1) as usize
}
