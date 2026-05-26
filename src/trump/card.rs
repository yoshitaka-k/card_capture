use crate::trump::constants::{
    SUIT_STR_HART,
    SUIT_STR_DIAMOND,
    SUIT_STR_CLOVER,
    SUIT_STR_SPADE,
    SUIT_STR_JOKER,

    SUIT_ICON_HART,
    SUIT_ICON_DIAMOND,
    SUIT_ICON_CLOVER,
    SUIT_ICON_SPADE,
    SUIT_ICON_JOKER,

    ACE_STR_RANK,
    ACE_FROM_RANK,
    ACE_TO_RANK,

    JACK_STR_RANK,
    JACK_FROM_RANK,
    JACK_TO_RANK,

    QUEEN_STR_RANK,
    QUEEN_FROM_RANK,
    QUEEN_TO_RANK,

    KING_STR_RANK,
    KING_FROM_RANK,
    KING_TO_RANK,
};

/// カードの情報
#[derive(Debug, Clone)]
pub struct Card {
    suit: String,
    rank: usize,
}

impl Card {
    pub fn new(suit: &str, rank: usize) -> Self {
        Self { suit: suit.to_string(), rank }
    }

    pub fn get_suit(&self) -> &String {
        &self.suit
    }

    pub fn get_rank(&self) -> usize {
        self.rank
    }

    /// 手札表示用の並び: スート（h → d → c → s → j）、同スート内はランクの数値順。
    pub fn sort_tuple(&self) -> (u8, u16) {
        let suit = match self.suit.as_str() {
            SUIT_STR_HART => 0,
            SUIT_STR_DIAMOND => 1,
            SUIT_STR_CLOVER => 2,
            SUIT_STR_SPADE => 3,
            SUIT_STR_JOKER => 4,
            _ => 5,
        };
        let rank = self.rank as u16;
        (suit, rank)
    }

    pub fn get_calc_rank(&self) -> usize {
        match self.rank {
            JACK_FROM_RANK => JACK_TO_RANK,
            QUEEN_FROM_RANK => QUEEN_TO_RANK,
            KING_FROM_RANK => KING_TO_RANK,
            ACE_FROM_RANK => ACE_TO_RANK,
            _ => self.rank,
        }
    }

    pub fn get_disp_rank(&self) -> String {
        match self.rank {
            ACE_FROM_RANK => ACE_STR_RANK.to_string(),
            JACK_FROM_RANK => JACK_STR_RANK.to_string(),
            QUEEN_FROM_RANK => QUEEN_STR_RANK.to_string(),
            KING_FROM_RANK => KING_STR_RANK.to_string(),
            _ => self.rank.to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        let suit = match self.suit.as_str() {
            SUIT_STR_HART => SUIT_ICON_HART,
            SUIT_STR_DIAMOND => SUIT_ICON_DIAMOND,
            SUIT_STR_CLOVER => SUIT_ICON_CLOVER,
            SUIT_STR_SPADE => SUIT_ICON_SPADE,
            SUIT_STR_JOKER => SUIT_ICON_JOKER,
            _ => "?",
        };

        format!("{}{}", suit, self.get_disp_rank())
    }
}

impl std::fmt::Display for Card {
    /// スート・ランク表示
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
