use crate::trump::{Card};

/// プレイヤー毎の手札構造体
#[derive(Default)]
pub struct CardSet(Vec<Card>);
impl CardSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn add_to_index(&mut self, index: usize, card: Card) {
        self.0.insert(index, card);
    }

    pub fn remove(&mut self, index: usize) -> Card {
        self.0.remove(index)
    }

    pub fn get(&mut self) -> &mut Vec<Card> {
        &mut self.0
    }

    pub fn get_card(&self, index: usize) -> Option<&Card> {
        let cardset = &self.0;
        if index < cardset.len() {
            Some(&cardset[index])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn sort(&mut self) {
        self.0.sort_by_key(|c| c.sort_tuple());
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
