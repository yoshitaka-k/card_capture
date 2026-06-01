use getset::{MutGetters, Setters};
use crate::trump::{Card};

pub enum DeckType {
    Enemy,
    Player,
}

/// トランプカードの山札
#[derive(Default, MutGetters, Setters)]
pub struct Deck {
    #[getset(get_mut = "pub", set = "pub")]
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(deck_type: DeckType) -> Self {
        let mut cards = Vec::new();

        match deck_type {
            DeckType::Player => {
                for suit in ["h", "d", "c", "s"] {
                    for rank in [2, 3, 4] {
                        cards.push(Card::new(suit, rank));
                    }
                }
                cards.push(Card::new("j", 0));
                cards.push(Card::new("j", 1));
            }
            DeckType::Enemy => {
                for suit in ["h", "d", "c", "s"] {
                    for rank in [1, 5, 6, 7, 8, 9, 10, 11, 12, 13] {
                        cards.push(Card::new(suit, rank));
                    }
                }
            }
        }

        Self { cards: cards }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
}
