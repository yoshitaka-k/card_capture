use crate::constants::{MAX_HAND_SIZE};
use crate::trump::constants::{
    JACK_FROM_RANK,
    QUEEN_FROM_RANK,
    KING_FROM_RANK,
    ACE_FROM_RANK,
    JOKER_FROM_RANK,
};
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
    enemy_suit: String,
    player_suit: String,
    enemy_cupture: bool,
    player_cupture: bool,
    discard: bool,
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

    /// プレイヤーのデッキにカードを設定
    pub fn set_player_deck_cards(&mut self, cards: Vec<Card>) {
        self.player_deck.set_cards(cards);
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

    /// 敵の手札からカードをデッキに戻す
    fn put_enemy_card_to_deck(&mut self, index: usize) {
        if let Some(card) = self.enemy_hand.take(index) {
            self.enemy_deck.push(card);
        }
    }

    /// ゲーム開始終了時の敵のデッキを初期化
    pub fn initial_end_phase_enemy_deck(&mut self) {
        for i in 0..MAX_HAND_SIZE {
            if let Some(card) = self.enemy_hand.get_card(i) {
                match card.get_rank() {
                    JACK_FROM_RANK => self.put_enemy_card_to_deck(i),
                    QUEEN_FROM_RANK => self.put_enemy_card_to_deck(i),
                    KING_FROM_RANK => self.put_enemy_card_to_deck(i),
                    ACE_FROM_RANK => self.put_enemy_card_to_deck(i),
                    JOKER_FROM_RANK => self.put_enemy_card_to_deck(i),
                    _ => continue,
                };
            } else {
                continue;
            }
        }
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

    /// 敵の手札からカードを取り出す
    pub fn take_enemy_hand_card(&mut self, index: usize) {
        self.enemy_hand.take(index);
    }

    /// プレイヤーの手札からカードを取り出す
    pub fn take_player_hand_card(&mut self, index: usize) {
        self.player_hand.take(index);
    }

    /// 敵の手札からカードを削除
    pub fn remove_enemy_hand_card(&mut self, index: usize) {
        self.enemy_hand.remove(index);
    }

    /// 敵の手札から `None` の空きスロットを除去する
    pub fn compact_enemy_hand(&mut self) {
        self.enemy_hand.compact();
    }

    /// プレイヤーの手札から `None` の空きスロットを除去する
    pub fn compact_player_hand(&mut self) {
        self.player_hand.compact();
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

    /// 敵のスートを設定
    pub fn set_enemy_suit(&mut self, suit: &str) {
        self.enemy_suit = suit.to_string();
    }

    /// プレイヤーのスートを設定
    pub fn set_player_suit(&mut self, suit: &str) {
        self.player_suit = suit.to_string();
    }

    /// 敵のスートを取得
    pub fn get_enemy_suit(&self) -> &String {
        &self.enemy_suit
    }

    /// プレイヤーのスートを取得
    pub fn get_player_suit(&self) -> &String {
        &self.player_suit
    }

    /// 敵のスートをクリア
    pub fn clear_enemy_suit(&mut self) {
        self.enemy_suit = String::new();
    }

    /// プレイヤーのスートをクリア
    pub fn clear_player_suit(&mut self) {
        self.player_suit = String::new();
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

    /// 捨て札フラグを設定
    pub fn set_discard(&mut self, discard: bool) {
        self.discard = discard;
    }

    /// 捨て札フラグを取得
    pub fn is_discard(&self) -> bool {
        self.discard
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

    /// プレイヤーの捨て札をクリア
    pub fn clear_player_discard(&mut self) {
        self.player_discard.clear();
    }

    /// 敵の捨て札をクリア
    pub fn clear_enemy_discard(&mut self) {
        self.enemy_discard.clear();
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
