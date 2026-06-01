use getset::{Getters, Setters};
use crate::constants::{MAX_HAND_SIZE};
use crate::trump::constants::{
    JACK_FROM_RANK,
    QUEEN_FROM_RANK,
    KING_FROM_RANK,
    ACE_FROM_RANK,
    SUIT_STR_JOKER,
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

#[derive(Default, Getters, Setters)]
pub struct Game {
    #[getset(get = "pub")]
    enemy_deck: Deck,
    #[getset(get = "pub")]
    player_deck: Deck,

    #[getset(get = "pub")]
    enemy_hand: CardSet,
    #[getset(get = "pub")]
    player_hand: CardSet,

    player_hand_copy_joker: [Vec<bool>; 2],

    #[getset(get = "pub")]
    enemy_discard: Vec<Card>,
    #[getset(get = "pub")]
    player_discard: Vec<Card>,

    #[getset(get = "pub")]
    enemy_select: Vec<bool>,
    #[getset(get = "pub")]
    player_select: Vec<bool>,

    #[getset(set = "pub")]
    enemy_cupture: bool,
    #[getset(set = "pub")]
    player_cupture: bool,

    #[getset(set = "pub")]
    discard: bool,
    #[getset(set = "pub")]
    sacrifice: bool,

    #[getset(set = "pub")]
    gameover: bool,

    #[getset(get = "pub")]
    suit: String,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.enemy_deck = Deck::new(DeckType::Enemy);
        self.player_deck = Deck::new(DeckType::Player);

        self.enemy_discard = Vec::new();
        self.player_discard = Vec::new();

        self.enemy_hand = CardSet::new();
        self.player_hand = CardSet::new();
        self.player_hand_copy_joker = [vec![false; MAX_HAND_SIZE], vec![false; MAX_HAND_SIZE]];

        self.enemy_select = vec![false; MAX_HAND_SIZE];
        self.player_select = vec![false; MAX_HAND_SIZE];

        self.enemy_cupture = false;
        self.player_cupture = false;
        self.discard = false;
        self.sacrifice = false;
        self.gameover = false;
    }

    // --- デッキ ---

    /// プレイヤーのデッキにカードを設定
    pub fn set_player_deck_cards(&mut self, cards: Vec<Card>) {
        self.player_deck.set_cards(cards);
    }

    /// 敵のデッキをシャッフル
    pub fn shuffle_enemy_deck(&mut self) {
        shuffle_deck(self.enemy_deck.cards_mut());
    }

    /// プレイヤーのデッキをシャッフル
    pub fn shuffle_player_deck(&mut self) {
        shuffle_deck(self.player_deck.cards_mut());
    }

    /// 敵のデッキにカードを追加
    pub fn add_enemy_deck(&mut self, card: Card) {
        self.enemy_deck.push(card);
    }

    /// プレイヤーのデッキにカードを追加
    pub fn add_player_deck(&mut self, card: Card) {
        self.player_deck.push(card);
    }

    /// 敵のデッキからカードを引く
    pub fn draw_enemy_card(&mut self) -> Option<Card> {
        self.enemy_deck.draw()
    }

    /// プレイヤーのデッキからカードを引く
    pub fn draw_player_card(&mut self) -> Option<Card> {
        self.player_deck.draw()
    }

    /// 敵の手札からカードをデッキに戻す
    fn put_enemy_card_to_deck(&mut self, index: usize) {
        if let Some(card) = self.enemy_hand.take(index) {
            self.enemy_deck.push(card);
        }
    }

    /// ゲーム開始のセットアップフェース終了時の敵のデッキを初期化
    pub fn initial_end_phase_enemy_deck(&mut self) {
        for i in 0..MAX_HAND_SIZE {
            if let Some(card) = self.enemy_hand.card(i) {
                match card.rank() {
                    JACK_FROM_RANK => self.put_enemy_card_to_deck(i),
                    QUEEN_FROM_RANK => self.put_enemy_card_to_deck(i),
                    KING_FROM_RANK => self.put_enemy_card_to_deck(i),
                    ACE_FROM_RANK => self.put_enemy_card_to_deck(i),
                    _ => continue,
                };
            } else {
                continue;
            }
        }
    }

    // --- 手札 ---

    /// 敵の手札にカードを追加
    pub fn add_enemy_hand(&mut self, card: Card) {
        self.enemy_hand.add(card);
    }

    /// プレイヤーの手札にカードを追加
    pub fn add_player_hand(&mut self, card: Card) {
        self.player_hand.add(card);
    }

    /// 敵の手札からカードを取り出す
    pub fn take_enemy_hand_card(&mut self, index: usize) -> Option<Card> {
        self.enemy_hand.take(index)
    }

    /// プレイヤーの手札からカードを取り出す
    pub fn take_player_hand_card(&mut self, index: usize) -> Option<Card> {
        self.player_hand.take(index)
    }

    /// 敵の手札からカードを削除
    pub fn remove_enemy_hand_card(&mut self, index: usize) {
        self.enemy_hand.remove(index);
    }

    /// プレイヤーの手札からカードを削除
    pub fn remove_player_hand_card(&mut self, index: usize) {
        self.player_hand.remove(index);
    }

    /// 敵の手札から `None` の空きスロットを除去する
    pub fn compact_enemy_hand(&mut self) {
        self.enemy_hand.compact();
    }

    /// プレイヤーの手札から `None` の空きスロットを除去する
    pub fn compact_player_hand(&mut self) {
        self.player_hand.compact();
    }

    // --- ジョーカーコピー ---

    /// 手札のジョーカーに他カードのランクの有無を設定
    pub fn set_player_hand_copy_joker(&mut self, joker_index: usize, index: usize, selected: bool) {
        self.player_hand_copy_joker[joker_index][index] = selected;
    }

    /// 手札のジョーカーに他カードのランクの有無を取得
    pub fn is_player_hand_copy_joker(&self, joker_index: usize, index: usize) -> bool {
        self.player_hand_copy_joker
            .get(joker_index)
            .and_then(|slots| slots.get(index))
            .copied()
            .unwrap_or(false)
    }

    /// ジョーカーへコピー元として選ばれている手札インデックス
    pub fn player_hand_copy_joker_source(&self, joker_index: usize) -> Option<usize> {
        self.player_hand_copy_joker[joker_index]
            .iter()
            .position(|&selected| selected)
    }

    /// ジョーカーコピー用スロットから未選択のものを除去する
    pub fn compact_player_hand_copy_joker(&mut self, index: usize) {
        self.player_hand_copy_joker[index].retain(|joker| *joker);
    }

    /// プレイヤーの手札にジョーカーのランクをクリア
    pub fn clear_player_hand_copy_joker(&mut self, index: usize) {
        for joker in self.player_hand_copy_joker[index].iter_mut() {
            *joker = false;
        }
    }

    // --- 選択 ---

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

    /// プレイヤーの手札を選択し、未設定ならそのカードのスートを記録する
    pub fn select_player_hand(&mut self, index: usize) -> bool {
        if self.player_hand.card(index).is_none() {
            return false;
        }
        self.player_select[index] = true;
        self.set_suit_from_player_hand(index);
        true
    }

    /// 敵の手札を選択する。プレイヤーが未選択ならスートを再設定する
    pub fn select_enemy_hand(&mut self, index: usize) -> bool {
        if self.enemy_hand.card(index).is_none() {
            return false;
        }
        for i in 0..self.enemy_select.len() {
            self.enemy_select[i] = false;
        }
        self.enemy_select[index] = true;
        if self.player_select.iter().all(|&selected| !selected) {
            self.clear_suit();
            self.set_suit_from_enemy_hand(index);
        }
        true
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

    /// 敵の選択したカードの合計ランクを計算
    pub fn calc_enemy_select_rank(&self) -> usize {
        let mut rank = 0;
        for (index, selected) in self.enemy_select.iter().enumerate() {
            if *selected {
                if let Some(card) = self.enemy_hand.card(index) {
                    rank += card.calc_rank();
                }
            }
        }
        rank
    }

    /// プレイヤーの選択したカードの合計ランクを計算
    pub fn calc_player_select_rank(&self) -> usize {
        let mut rank = 0;
        // プレイヤーの選択したカードの合計ランクを計算
        for (index, selected) in self.player_select.iter().enumerate() {
            if *selected {
                if let Some(card) = self.player_hand.card(index) {
                    rank += card.calc_rank();
                }
            }
        }
        // コピーしたジョーカーのランクを加算
        for (index, selected) in self.player_hand_copy_joker[0].iter().enumerate() {
            if *selected {
                if let Some(card) = self.player_hand.card(index) {
                    rank += card.calc_rank();
                }
            }
        }
        rank
    }

    /// プレイヤーの選択したカードにジョーカーがあるかどうかを取得
    pub fn is_player_select_joker(&self) -> bool {
        for (index, selected) in self.player_select.iter().enumerate() {
            if *selected {
                if let Some(card) = self.player_hand.card(index) {
                    if card.is_joker() {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// 敵の選択状態をクリア
    pub fn clear_enemy_select(&mut self) {
        self.enemy_select = vec![false; MAX_HAND_SIZE];
    }

    /// プレイヤーの選択状態をクリア
    pub fn clear_player_select(&mut self) {
        self.player_select = vec![false; MAX_HAND_SIZE];
    }

    // --- フラグ ---

    /// 敵の捕獲状態を取得
    pub fn is_enemy_cupture(&self) -> bool {
        self.enemy_cupture
    }

    /// プレイヤーの捕獲状態を取得
    pub fn is_player_cupture(&self) -> bool {
        self.player_cupture
    }

    /// 捨て札フラグを取得
    pub fn is_discard(&self) -> bool {
        self.discard
    }

    /// 生贄フラグを取得
    pub fn is_sacrifice(&self) -> bool {
        self.sacrifice
    }

    /// ゲームオーバーフラグを取得
    pub fn is_gameover(&self) -> bool {
        self.gameover
    }

    // --- 捨て札 ---

    /// 敵の捨て札にカードを追加
    pub fn add_enemy_discard(&mut self, card: Card) {
        self.enemy_discard.push(card);
    }

    /// プレイヤーの捨て札にカードを追加
    pub fn add_player_discard(&mut self, card: Card) {
        self.player_discard.push(card);
    }

    /// 敵の捨て札をクリア
    pub fn clear_enemy_discard(&mut self) {
        self.enemy_discard.clear();
    }

    /// プレイヤーの捨て札をクリア
    pub fn clear_player_discard(&mut self) {
        self.player_discard.clear();
    }

    /// プレイヤーの捨て札を取り出す（山札切れ時の再構築用）
    pub fn take_player_discard(&mut self) -> Vec<Card> {
        std::mem::take(&mut self.player_discard)
    }

    // --- スート ---

    /// スートをクリア
    pub fn clear_suit(&mut self) {
        self.suit = String::new();
    }

    fn set_suit_from_player_hand(&mut self, index: usize) {
        if !self.suit.is_empty() {
            return;
        }
        if let Some(card) = self.player_hand.card(index) {
            if card.suit() != SUIT_STR_JOKER {
                self.suit.clone_from(card.suit());
            }
        }
    }

    fn set_suit_from_enemy_hand(&mut self, index: usize) {
        if !self.suit.is_empty() {
            return;
        }
        if let Some(card) = self.enemy_hand.card(index) {
            if card.suit() != SUIT_STR_JOKER {
                self.suit.clone_from(card.suit());
            }
        }
    }
}

/// デッキをシャッフル
fn shuffle_deck(cards: &mut Vec<Card>) {
    hindu_shuffle(cards, &HinduParams::default());
    riffle_shuffle(cards, &RiffleParams::default());
    deal_shuffle(cards, &DealParams::default());
    double_cut(cards);
}
