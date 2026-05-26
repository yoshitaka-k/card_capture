use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title Block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
            " App Title",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    // Content Block
    let text = format!(
        "Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
        app.counter
    );
    let content_text = Text::styled(text, Style::default().fg(Color::Green));

    let content = Paragraph::new(content_text)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(content, chunks[1]);

    // Current Navigation Text
    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled(" Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Exiting => Span::styled(" Exiting Mode", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            Span::styled("Hello!!", Style::default().fg(Color::DarkGray))
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                " (q) to quit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                " (q) to quit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    // exit popup window
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area());

        let popup_block = Block::default()
            .title("Quit App")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray));

        let ext_text = Text::styled(
            "\n\nWould you like to quit the app? (y/n)",
            Style::default().fg(Color::Red),
        );

        let ext_paragraph = Paragraph::new(ext_text)
            .block(popup_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        let area = centered_rect(40, 14, frame.area());
        frame.render_widget(ext_paragraph, area);
    }
}

// Helper function to center a rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
