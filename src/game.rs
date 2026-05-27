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
    enemy_select: Option<Card>,
    player_select: Vec<Option<Card>>,
    enemy_discard: Vec<Card>,
    player_discard: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.enemy_deck = Deck::new(DeckType::Enemy);
        self.player_deck = Deck::new(DeckType::Player);
        self.player_select = vec![None::<Card>; MAX_HAND_SIZE];
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
    pub fn add_enemy_select(&mut self, card: Option<Card>) {
        self.enemy_select = card;
    }

    /// プレイヤーの選択したカードを追加
    pub fn add_player_select(&mut self, index: usize, card: Option<Card>) {
        self.player_select[index] = card;
    }

    /// 敵の選択したカードを取得
    pub fn get_enemy_select(&self) -> Option<&Card> {
        if let Some(card) = &self.enemy_select {
            Some(card)
        } else {
            None
        }
    }

    /// プレイヤーの選択したカードを取得
    pub fn get_player_select(&self) -> &Vec<Option<Card>> {
        &self.player_select
    }

    /// プレイヤーの選択したカードを取得
    pub fn get_player_select_card(&self, index: usize) -> Option<&Card> {
        if index < self.player_select.len() {
            if let Some(card) = &self.player_select[index] {
                Some(card)
            } else {
                None
            }
        } else {
            None
        }
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
