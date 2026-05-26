use crate::trump::{Deck, DeckType, Hand};

#[derive(Default)]
pub struct Game {
    pub enemy_deck: Deck,
    pub player_deck: Deck,
    pub enemy_hand: Hand,
    pub player_hand: Hand,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.enemy_deck = Deck::new(DeckType::Enemy);
        self.player_deck = Deck::new(DeckType::Player);
    }

    pub fn draw_enemy_card(&mut self) {
        self.enemy_deck.draw().unwrap();
    }

    pub fn draw_player_card(&mut self) {
        self.player_deck.draw().unwrap();
    }
}
