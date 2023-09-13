//! binsider

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;

/// Terminal user interface.
pub mod tui;

/// ELF helper.
pub mod elf;

/// Command-line arguments parser.
pub mod args;

/// Error handler implementation.
pub mod error;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use app::Analyzer;
use args::Args;
use error::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tui::event::{Event, EventHandler};
use tui::handler::handle_key_events;
use tui::state::State;
use tui::Tui;

/// Starts the terminal user interface.
pub fn start_tui(analyzer: Analyzer, args: &Args) -> Result<()> {
    // Create an application.
    let mut state = State::new(analyzer);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    state
        .analyzer
        .extract_strings(args.min_strings_len, events.sender.clone());
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while state.running {
        // Render the user interface.
        tui.draw(&mut state)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => state.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut state)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::FileStrings(strings) => {
                state.analyzer.strings = Some(strings?);
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
