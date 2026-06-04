use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Text,
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};
use figlet_rs::FIGlet;

use crate::app::{App, GamePhase};

const TITLE_END_SPINNER: [char; 4] = ['|', '/', '-', '\\'];

pub fn render_title_block(app: &mut App, frame: &mut Frame, area: Rect) {
    app.positions.clear();

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

    // Title Title
    let figlet = FIGlet::standard().unwrap();
    let title = &format!("{}", figlet.convert("Card Capture").unwrap());

    let paragraph = Paragraph::new(Text::from(title.clone()))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green).bold());

    frame.render_widget(paragraph, content_chunks[1]);

    // Title Text
    let text = format!("------------------------------------------------------------\nVersion: {}  |  License: {}\nStarting {} Game Engine... 🚀",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_LICENSE"),
        env!("CARGO_PKG_NAME")
    );

    let paragraph = Paragraph::new(Text::from(text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(paragraph, content_chunks[2]);

    // Title Text
    let text = if app.current_phase == GamePhase::TitleEnd {
        let spinner = TITLE_END_SPINNER[app.title_end_spinner_frame()];
        format!(
            "--------------------------------\n{spinner} Starting game...🚀\n--------------------------------"
        )
    } else {
        "--------------------------------\nClick to start\n--------------------------------".to_string()
    };
    let paragraph = Paragraph::new(Text::from(text))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    frame.render_widget(paragraph, content_chunks[3]);
    app.positions.set_title(content_chunks[3]);
}
