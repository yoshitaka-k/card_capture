use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use color_eyre::{Result};

use card_capture::constants::TICK_RATE_MILLIS;
use card_capture::{
    app::App,
    event::{Event, EventHandler},
    tui::Tui,
    update::{key_update, mouse_update},
};

fn main() -> Result<()> {
    let mut app = App::new();

    app.start();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MILLIS);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    tui.draw(&mut app)?;

    while !app.should_quit {
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => key_update(&mut app, key_event),
            Event::Mouse(mouse_event) => {
                mouse_update(&mut app, mouse_event);
            }
        };
        tui.draw(&mut app)?;
    }

    tui.exit()?;
    Ok(())
}
