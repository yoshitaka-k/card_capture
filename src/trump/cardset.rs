use crate::trump::{Card};

/// プレイヤー毎の手札構造体
#[derive(Default)]
pub struct CardSet(Vec<Option<Card>>);
impl CardSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, card: Card) {
        self.0.push(Some(card));
    }

    pub fn add_to_index(&mut self, index: usize, card: Card) {
        self.0.insert(index, Some(card));
    }

    pub fn take(&mut self, index: usize) -> Option<Card> {
        if index < self.0.len() {
            self.0.get_mut(index)?.take()
        } else {
            None
        }
    }

    pub fn get(&mut self) -> &mut Vec<Option<Card>> {
        &mut self.0
    }

    pub fn get_card(&self, index: usize) -> Option<&Card> {
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

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
