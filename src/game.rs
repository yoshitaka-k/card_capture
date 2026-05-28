use crate::constants::MAX_HAND_SIZE;
use crate::trump::{Card, Deck, DeckType, CardSet};
use crate::trump::shuffle::{
    hindu_shuffle,
    riffle_shuffle,
    deal_shuffle,
    double_cut,
    HinduParams,
    RiffleParams,
    DealParams,
};

#[derive(Default)]
pub struct Game {
    enemy_deck: Deck,
    player_deck: Deck,
    enemy_hand: CardSet,
    player_hand: CardSet,
    enemy_discard: Vec<Card>,
    player_discard: Vec<Card>,
    enemy_select: Vec<bool>,
    player_select: Vec<bool>,
    enemy_cupture: bool,
    player_cupture: bool,
    sacrifice: bool,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.enemy_deck = Deck::new(DeckType::Enemy);
        self.player_deck = Deck::new(DeckType::Player);
        self.enemy_select = vec![false; MAX_HAND_SIZE];
        self.player_select = vec![false; MAX_HAND_SIZE];
    }

    /// 敵のデッキをシャッフル
    pub fn shuffle_enemy_deck(&mut self) {
        let cards = self.enemy_deck.get_cards();
        shuffle_deck(cards);
    }

    /// プレイヤーのデッキをシャッフル
    pub fn shuffle_player_deck(&mut self) {
        let cards = self.player_deck.get_cards();
        shuffle_deck(cards);
    }

    /// 敵のデッキを取得
    pub fn get_enemy_deck(&self) -> &Deck {
        &self.enemy_deck
    }

    /// プレイヤーのデッキを取得
    pub fn get_player_deck(&self) -> &Deck {
        &self.player_deck
    }

    /// 敵のデッキにカードを追加
    pub fn add_enemy_deck(&mut self, card: Card) {
        self.enemy_deck.push(card);
    }

    /// プレイヤーのデッキにカードを追加
    pub fn add_player_deck(&mut self, card: Card) {
        self.player_deck.push(card);
    }

    /// 敵の手札にカードを追加
    pub fn add_enemy_hand(&mut self, card: Card) {
        self.enemy_hand.add(card);
    }

    /// プレイヤーの手札にカードを追加
    pub fn add_player_hand(&mut self, card: Card) {
        self.player_hand.add(card);
    }

    /// 敵の手札を取得
    pub fn get_enemy_hand(&self) -> &CardSet {
        &self.enemy_hand
    }

    /// 敵の手札を取得
    pub fn get_enemy_hand_card(&self, index: usize) -> Option<&Card> {
        if index < self.enemy_hand.len() {
            self.enemy_hand.get_card(index)
        } else {
            None
        }
    }

    /// 敵の手札からカードを削除
    pub fn remove_enemy_hand_card(&mut self, index: usize) {
        self.enemy_hand.remove(index);
    }

    /// プレイヤーの手札からカードを削除
    pub fn remove_player_hand_card(&mut self, index: usize) {
        self.player_hand.remove(index);
    }

    /// プレイヤーの手札を取得
    pub fn get_player_hand(&self) -> &CardSet {
        &self.player_hand
    }

    /// プレイヤーの手札を取得
    pub fn get_player_hand_card(&self, index: usize) -> Option<&Card> {
        if index < self.player_hand.len() {
            self.player_hand.get_card(index)
        } else {
            None
        }
    }

    /// 敵の選択したカードを追加
    pub fn add_enemy_select(&mut self, index: usize, selected: bool) {
        for i in 0..self.enemy_select.len() {
            self.enemy_select[i] = false;
        }
        self.enemy_select[index] = selected;
    }

    /// プレイヤーの選択したカードを追加
    pub fn add_player_select(&mut self, index: usize, selected: bool) {
        self.player_select[index] = selected;
    }

    /// 敵の選択したカードを取得
    pub fn get_enemy_select(&self) -> &Vec<bool> {
        &self.enemy_select
    }

    /// プレイヤーの選択したカードを取得
    pub fn get_player_select(&self) -> &Vec<bool> {
        &self.player_select
    }

    /// 敵の選択状態を取得
    pub fn is_enemy_selected(&self, index: usize) -> bool {
        if index < self.enemy_select.len() {
            self.enemy_select[index]
        } else {
            false
        }
    }
    /// プレイヤーの選択状態を取得
    pub fn is_player_selected(&self, index: usize) -> bool {
        if index < self.player_select.len() {
            self.player_select[index]
        } else {
            false
        }
    }

    /// プレイヤーの選択したカードの合計ランクを計算
    pub fn calc_player_select_rank(&self) -> usize {
        let mut rank = 0;
        for (index, selected) in self.player_select.iter().enumerate() {
            if *selected {
                if let Some(card) = self.player_hand.get_card(index) {
                    rank += card.get_calc_rank();
                }
            }
        }
        rank
    }

    /// 敵の選択したカードの合計ランクを計算
    pub fn calc_enemy_select_rank(&self) -> usize {
        let mut rank = 0;
        for (index, selected) in self.enemy_select.iter().enumerate() {
            if *selected {
                if let Some(card) = self.enemy_hand.get_card(index) {
                    rank += card.get_calc_rank();
                }
            }
        }
        rank
    }

    /// 敵の選択状態をクリア
    pub fn clear_enemy_select(&mut self) {
        self.enemy_select = vec![false; MAX_HAND_SIZE];
    }

    /// プレイヤーの選択状態をクリア
    pub fn clear_player_select(&mut self) {
        self.player_select = vec![false; MAX_HAND_SIZE];
    }

    /// 敵の捕獲状態を設定
    pub fn set_enemy_cupture(&mut self, cupture: bool) {
        self.enemy_cupture = cupture;
    }

    /// プレイヤーの捕獲状態を設定
    pub fn set_player_cupture(&mut self, cupture: bool) {
        self.player_cupture = cupture;
    }

    /// 敵の捕獲状態を取得
    pub fn is_enemy_cupture(&self) -> bool {
        self.enemy_cupture
    }

    /// プレイヤーの捕獲状態を取得
    pub fn is_player_cupture(&self) -> bool {
        self.player_cupture
    }

    /// 生贄フラグを設定
    pub fn set_sacrifice(&mut self, sacrifice: bool) {
        self.sacrifice = sacrifice;
    }

    /// 生贄フラグを取得
    pub fn is_sacrifice(&self) -> bool {
        self.sacrifice
    }

    /// 敵の捨て札にカードを追加
    pub fn add_enemy_discard(&mut self, card: Card) {
        self.enemy_discard.push(card);
    }

    /// プレイヤーの捨て札にカードを追加
    pub fn add_player_discard(&mut self, card: Card) {
        self.player_discard.push(card);
    }

    /// 敵の捨て札を取得
    pub fn get_enemy_discard(&self) -> &Vec<Card> {
        &self.enemy_discard
    }

    /// プレイヤーの捨て札を取得
    pub fn get_player_discard(&self) -> &Vec<Card> {
        &self.player_discard
    }

    /// 敵のデッキからカードを引く
    pub fn draw_enemy_card(&mut self) -> Option<Card> {
        self.enemy_deck.draw()
    }

    /// プレイヤーのデッキからカードを引く
    pub fn draw_player_card(&mut self) -> Option<Card> {
        self.player_deck.draw()
    }
}

/// デッキをシャッフル
fn shuffle_deck(cards: &mut Vec<Card>) {
    hindu_shuffle(cards, &HinduParams::default());
    riffle_shuffle(cards, &RiffleParams::default());
    deal_shuffle(cards, &DealParams::default());
    double_cut(cards);
}
