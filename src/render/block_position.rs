use ratatui::layout::Rect;
use getset::{Getters, Setters};

/// ブロックの位置を管理する構造体
#[derive(Default, Getters, Setters)]
pub struct BlockPosition {
    #[getset(get = "pub", set = "pub")]
    enemy_deck: Rect,
    #[getset(get = "pub", set = "pub")]
    player_deck: Rect,

    #[getset(get = "pub")]
    enemy_hand: Vec<Rect>,
    #[getset(get = "pub")]
    player_hand: Vec<Rect>,

    #[getset(get = "pub")]
    player_hand_copy_joker: Vec<Rect>,

    #[getset(get = "pub", set = "pub")]
    enemy_discard: Rect,
    #[getset(get = "pub", set = "pub")]
    player_discard: Rect,
    #[getset(get = "pub", set = "pub")]
    gameover: Rect,
}

impl BlockPosition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn add_player_hand(&mut self, rect: Rect) {
        self.player_hand.push(rect);
    }

    pub fn add_player_hand_copy_joker(&mut self, rect: Rect) {
        self.player_hand_copy_joker.push(rect);
    }

    pub fn add_enemy_hand(&mut self, rect: Rect) {
        self.enemy_hand.push(rect);
    }

    pub fn remove_player_hand(&mut self, index: usize) {
        self.player_hand.remove(index);
    }

    pub fn remove_enemy_hand(&mut self, index: usize) {
        self.enemy_hand.remove(index);
    }

    pub fn remove_player_hand_copy_joker(&mut self, index: usize) {
        self.player_hand_copy_joker.remove(index);
    }
}
