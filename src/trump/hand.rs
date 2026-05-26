use crate::trump::{Card};

/// プレイヤー毎の手札構造体
#[derive(Default)]
pub struct Hand {
    cards: Vec<Card>,
}
impl Hand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_to_index(&mut self, index: usize, card: Card) {
        self.cards.insert(index, card);
    }

    pub fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    pub fn get(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn sort(&mut self) {
        self.cards.sort_by_key(|c| c.sort_tuple());
    }

    pub fn get_card(&self, index: usize) -> &Card {
        let cardset = &self.cards;
        &cardset[index]
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }
}
