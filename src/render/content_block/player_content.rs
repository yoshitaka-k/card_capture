use ratatui::layout::Rect;
use ratatui::Frame;

use crate::{app::App};
use crate::trump::deck::DeckType;
use crate::render::content_block::hand_area_layout;

pub fn player_content(app: &mut App, frame: &mut Frame, area: Rect) {
    hand_area_layout(app, frame, area, DeckType::Player);
}
