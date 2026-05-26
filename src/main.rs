use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use color_eyre::{Result};

/// Application
pub mod app;
use app::App;
/// Event
pub mod event;
use event::{Event, EventHandler};
/// Widget
pub mod ui;
/// user interface
pub mod tui;
use tui::Tui;
/// Application updater
pub mod update;
use update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
