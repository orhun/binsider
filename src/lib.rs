//! binsider

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;

/// Terminal user interface.
pub mod tui;

/// Command-line arguments parser.
pub mod args;

/// Error handler implementation.
pub mod error;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use error::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tui::event::{Event, EventHandler};
use tui::handler::handle_key_events;
use tui::state::State;
use tui::Tui;

/// Starts the terminal user interface.
pub fn start_tui() -> Result<()> {
    // Create an application.
    let mut app = State::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
