use crate::trump::{Card};

/// プレイヤー毎の手札構造体
#[derive(Default)]
pub struct CardSet(Vec<Option<Card>>);
impl CardSet {
    /// 敵の手札を初期化
    pub fn new() -> Self {
        Self::default()
    }

    /// 敵の手札にカードを追加
    pub fn add(&mut self, card: Card) {
        self.0.push(Some(card));
    }

    /// 敵の手札にカードを追加
    pub fn add_to_index(&mut self, index: usize, card: Card) {
        self.0.insert(index, Some(card));
    }

    /// 敵の手札からカードを取り出す
    pub fn take(&mut self, index: usize) -> Option<Card> {
        if index < self.0.len() {
            self.0.get_mut(index)?.take()
        } else {
            None
        }
    }

    /// 敵の手札からカードを削除
    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }

    /// 敵の手札を取得
    pub fn get(&mut self) -> &mut Vec<Option<Card>> {
        &mut self.0
    }

    /// 敵の手札からカードを取得
    pub fn card(&self, index: usize) -> Option<&Card> {
        let cardset = &self.0;
        if index < cardset.len() {
            if let Some(card) = &cardset[index] {
                Some(&card)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 敵の手札の長さを取得
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 敵の手札をクリア
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// `None` の空きスロットを詰めて、カードだけを左詰めする
    pub fn compact(&mut self) {
        self.0.retain(|card| card.is_some());
    }

    /// 手札にジョーカーがあるかどうかを取得
    pub fn is_joker(&self, index: usize) -> bool {
        if let Some(card) = self.card(index) {
            card.is_joker()
        } else {
            false
        }
    }

    /// 手札にジョーカーがあるかどうかを取得
    pub fn has_joker(&self) -> bool {
        if self.0.len() == 0 {
            return false;
        }
        for card in self.0.iter() {
            if let Some(card) = card {
                if card.is_joker() {
                    return true;
                }
            }
        }
        false
    }

    // 手札のジョーカーのカードを取得
    pub fn joker_cards(&self) -> Vec<&Card> {
        let mut joker_cards = Vec::new();
        for card in self.0.iter() {
            if let Some(card) = card {
                if card.is_joker() {
                    joker_cards.push(card);
                }
            }
        }
        joker_cards
    }
}
