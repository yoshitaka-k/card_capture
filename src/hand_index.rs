use crate::constants::MAX_HAND_SIZE;

/// 見た目のインデックス（左→右）を手札インデックス（右詰め）に変換する
#[inline]
pub fn visual_to_hand_index(visual_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - visual_index
}

/// 手札インデックス（右詰め）を見た目のインデックス（左→右）に変換する
#[inline]
pub fn hand_to_visual_index(hand_index: usize) -> usize {
    MAX_HAND_SIZE - 1 - hand_index
}
