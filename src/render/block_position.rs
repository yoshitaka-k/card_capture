use ratatui::layout::Rect;

/// ブロックの位置を管理する構造体
#[derive(Default)]
pub struct BlockPosition {
    enemy_deck: Rect,
    player_deck: Rect,
    enemy_hand: Vec<Rect>,
    player_hand: Vec<Rect>,
    player_hand_copy_joker: Vec<Rect>,
    enemy_discard: Rect,
    player_discard: Rect,
}

impl BlockPosition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn set_player_deck(&mut self, rect: Rect) {
        self.player_deck = rect;
    }

    pub fn set_enemy_deck(&mut self, rect: Rect) {
        self.enemy_deck = rect;
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

    pub fn set_player_discard(&mut self, rect: Rect) {
        self.player_discard = rect;
    }

    pub fn set_enemy_discard(&mut self, rect: Rect) {
        self.enemy_discard = rect;
    }

    pub fn get_player_deck(&self) -> &Rect {
        &self.player_deck
    }

    pub fn get_enemy_deck(&self) -> &Rect {
        &self.enemy_deck
    }

    pub fn get_enemy_hand(&self) -> &Vec<Rect> {
        &self.enemy_hand
    }

    pub fn get_player_hand(&self) -> &Vec<Rect> {
        &self.player_hand
    }

    pub fn get_player_hand_copy_joker(&self) -> &Vec<Rect> {
        &self.player_hand_copy_joker
    }

    pub fn get_player_discard(&self) -> &Rect {
        &self.player_discard
    }

    pub fn get_enemy_discard(&self) -> &Rect {
        &self.enemy_discard
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
