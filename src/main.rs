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
    app.game.shuffle_enemy_deck();
    app.game.shuffle_player_deck();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE_MILLIS);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => key_update(&mut app, key_event),
            Event::Mouse(mouse_event) => {
                mouse_update(&mut app, mouse_event);
            }
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
