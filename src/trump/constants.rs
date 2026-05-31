/// スート文字
pub const SUIT_STR_HART: &str = "h";
pub const SUIT_STR_DIAMOND: &str = "d";
pub const SUIT_STR_CLOVER: &str = "c";
pub const SUIT_STR_SPADE: &str = "s";
pub const SUIT_STR_JOKER: &str = "j";

/// スート記号
pub const SUIT_ICON_HART: &str = "♥";
pub const SUIT_ICON_DIAMOND: &str = "♦";
pub const SUIT_ICON_CLOVER: &str = "♣";
pub const SUIT_ICON_SPADE: &str = "♠";
pub const SUIT_ICON_JOKER: &str = "J";

/// エースのランク（ゲーム内では 11 としても扱う）
pub const ACE_FROM_RANK: usize = 1;
pub const ACE_TO_RANK: usize = 14;
pub const ACE_STR_RANK: &str = "A";

/// 絵札のランク（ゲーム内では 10 として扱う）
pub const JACK_FROM_RANK: usize = 11;
pub const JACK_TO_RANK: usize = 11;
pub const JACK_STR_RANK: &str = "J";

pub const QUEEN_FROM_RANK: usize = 12;
pub const QUEEN_TO_RANK: usize = 12;
pub const QUEEN_STR_RANK: &str = "Q";

pub const KING_FROM_RANK: usize = 13;
pub const KING_TO_RANK: usize = 13;
pub const KING_STR_RANK: &str = "K";

pub const JOKER1_FROM_RANK: usize = 0;
pub const JOKER2_FROM_RANK: usize = 1;
pub const JOKER_TO_RANK: usize = 0;
pub const JOKER_STR_RANK: &str = "";
