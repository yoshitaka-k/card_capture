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
    enemy_trash: CardSet,
    player_trash: CardSet,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.enemy_deck = Deck::new(DeckType::Enemy);
        self.player_deck = Deck::new(DeckType::Player);
        self.enemy_hand = CardSet::new();
        self.player_hand = CardSet::new();
        self.enemy_trash = CardSet::new();
        self.player_trash = CardSet::new();
    }

    pub fn shuffle_enemy_deck(&mut self) {
        let cards = self.enemy_deck.get_cards();
        shuffle_deck(cards);
    }

    pub fn shuffle_player_deck(&mut self) {
        let cards = self.player_deck.get_cards();
        shuffle_deck(cards);
    }

    pub fn get_enemy_deck(&self) -> &Deck {
        &self.enemy_deck
    }

    pub fn get_player_deck(&self) -> &Deck {
        &self.player_deck
    }

    pub fn add_enemy_hand(&mut self, card: Card) {
        self.enemy_hand.add(card);
    }

    pub fn add_player_hand(&mut self, card: Card) {
        self.player_hand.add(card);
    }

    pub fn get_enemy_hand(&self) -> &CardSet {
        &self.enemy_hand
    }

    pub fn get_player_hand(&self) -> &CardSet {
        &self.player_hand
    }

    pub fn add_enemy_trash(&mut self, card: Card) {
        self.enemy_trash.add(card);
    }

    pub fn add_player_trash(&mut self, card: Card) {
        self.player_trash.add(card);
    }

    pub fn get_enemy_trash(&self) -> &CardSet {
        &self.enemy_trash
    }

    pub fn get_player_trash(&self) -> &CardSet {
        &self.player_trash
    }

    pub fn draw_enemy_card(&mut self) -> Option<Card> {
        self.enemy_deck.draw()
    }

    pub fn draw_player_card(&mut self) -> Option<Card> {
        self.player_deck.draw()
    }
}

fn shuffle_deck(cards: &mut Vec<Card>) {
    hindu_shuffle(cards, &HinduParams::default());
    riffle_shuffle(cards, &RiffleParams::default());
    deal_shuffle(cards, &DealParams::default());
    double_cut(cards);
}
