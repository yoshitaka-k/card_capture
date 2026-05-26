use ratatui::layout::Rect;

/// ブロックの位置を管理する構造体
#[derive(Default)]
pub struct BlockPosition {
    player_deck: Rect,
    enemy_deck: Rect,
    player_hands: Vec<Rect>,
    enemy_hands: Vec<Rect>,
    player_trash: Rect,
    enemy_trash: Rect,
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
        self.player_hands.push(rect);
    }

    pub fn add_enemy_hand(&mut self, rect: Rect) {
        self.enemy_hands.push(rect);
    }

    pub fn set_player_trash(&mut self, rect: Rect) {
        self.player_trash = rect;
    }

    pub fn set_enemy_trash(&mut self, rect: Rect) {
        self.enemy_trash = rect;
    }

    pub fn get_player_deck(&self) -> &Rect {
        &self.player_deck
    }

    pub fn get_enemy_deck(&self) -> &Rect {
        &self.enemy_deck
    }

    pub fn get_enemy_hand(&self, index: usize) -> &Rect {
        &self.enemy_hands[index]
    }

    pub fn get_player_hand(&self, index: usize) -> &Rect {
        &self.player_hands[index]
    }

    pub fn get_player_trash(&self) -> &Rect {
        &self.player_trash
    }

    pub fn get_enemy_trash(&self) -> &Rect {
        &self.enemy_trash
    }

    pub fn get_player_hands(&self) -> &Vec<Rect> {
        &self.player_hands
    }

    pub fn get_enemy_hands(&self) -> &Vec<Rect> {
        &self.enemy_hands
    }

    pub fn remove_player_hand(&mut self, index: usize) {
        self.player_hands.remove(index);
    }

    pub fn remove_enemy_hand(&mut self, index: usize) {
        self.enemy_hands.remove(index);
    }
}
