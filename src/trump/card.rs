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

    JOKER_STR_RANK,
    JOKER_TO_RANK,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hart,
    Diamond,
    Clover,
    Spade,
    Joker,
}

impl Suit {
    pub fn from_str(suit: &str) -> Self {
        match suit {
            SUIT_STR_HART => Self::Hart,
            SUIT_STR_DIAMOND => Self::Diamond,
            SUIT_STR_CLOVER => Self::Clover,
            SUIT_STR_SPADE => Self::Spade,
            SUIT_STR_JOKER => Self::Joker,
            _ => Self::Joker,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hart => SUIT_STR_HART,
            Self::Diamond => SUIT_STR_DIAMOND,
            Self::Clover => SUIT_STR_CLOVER,
            Self::Spade => SUIT_STR_SPADE,
            Self::Joker => SUIT_STR_JOKER,
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::Hart => SUIT_ICON_HART,
            Self::Diamond => SUIT_ICON_DIAMOND,
            Self::Clover => SUIT_ICON_CLOVER,
            Self::Spade => SUIT_ICON_SPADE,
            Self::Joker => SUIT_ICON_JOKER,
        }
    }
}

/// カードの情報
#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit,
    rank: usize,
}

impl Card {
    pub fn new(suit: Suit, rank: usize) -> Self {
        Self { suit, rank }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn rank(&self) -> usize {
        self.rank
    }

    pub fn equals(&self, other: &Card) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }

    pub fn is_joker(&self) -> bool {
        self.suit == Suit::Joker
    }

    pub fn is_ace_card(&self) -> bool {
        self.rank == ACE_FROM_RANK
    }

    pub fn is_face_card(&self) -> bool {
        self.rank == JACK_FROM_RANK
        || self.rank == QUEEN_FROM_RANK
        || self.rank == KING_FROM_RANK
    }

    /// 手札表示用の並び: スート（h → d → c → s → j）、同スート内はランクの数値順。
    pub fn sort_tuple(&self) -> (u8, u16) {
        let suit = match self.suit {
            Suit::Hart => 0,
            Suit::Diamond => 1,
            Suit::Clover => 2,
            Suit::Spade => 3,
            Suit::Joker => 4,
        };
        let rank = self.rank as u16;
        (suit, rank)
    }

    pub fn calc_rank(&self) -> usize {
        match self.suit {
            Suit::Joker => {
                JOKER_TO_RANK
            }
            _ => {
                match self.rank {
                    JACK_FROM_RANK => JACK_TO_RANK,
                    QUEEN_FROM_RANK => QUEEN_TO_RANK,
                    KING_FROM_RANK => KING_TO_RANK,
                    ACE_FROM_RANK => ACE_TO_RANK,
                    _ => self.rank,
                }
            }
        }
    }

    pub fn disp_suit(&self) -> &'static str {
        self.suit.icon()
    }

    pub fn disp_rank(&self) -> &'static str {
        match self.suit {
            Suit::Joker => {
                JOKER_STR_RANK
            }
            _ => {
                match self.rank {
                    ACE_FROM_RANK => ACE_STR_RANK,
                    JACK_FROM_RANK => JACK_STR_RANK,
                    QUEEN_FROM_RANK => QUEEN_STR_RANK,
                    KING_FROM_RANK => KING_STR_RANK,
                    _ => "",
                }
            }
        }
    }

    pub fn name(&self) -> String {
        match self.disp_rank() {
            "" => format!("{}{}", self.disp_suit(), self.rank),
            rank => format!("{}{}", self.disp_suit(), rank),
        }
    }
}

impl std::fmt::Display for Card {
    /// スート・ランク表示
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.disp_rank() {
            "" => write!(f, "{}{}", self.disp_suit(), self.rank),
            rank => write!(f, "{}{}", self.disp_suit(), rank),
        }
    }
}
