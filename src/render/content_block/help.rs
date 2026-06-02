use ratatui::{
    layout::{Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Padding},
    Frame,
};

use crate::app::App;

/// Render the help content block
pub fn help_content(app: &mut App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Help Content")
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(1))
        .style(Style::default());

    let joker_help = app
        .game
        .selected_player_joker_ranks()
        .into_iter()
        .filter_map(|joker_rank| {
            app.game
                .player_hand_copy_joker_source(joker_rank)
                .and_then(|index| app.game.player_hand().card(index))
                .map(|card| format!("Joker{} from card: {}", joker_rank + 1, card.name()))
        })
        .collect::<Vec<_>>()
        .join(" / ");

    let text = if !joker_help.is_empty() {
        joker_help.as_str()
    } else if app.help_text.is_empty() {
        "Help content: Help text"
    } else {
        &app.help_text
    };

    let content = Paragraph::new(text).block(block);

    frame.render_widget(content, area);
}
