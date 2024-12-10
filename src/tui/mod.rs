/// Application state handler.
pub mod state;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Custom widgets.
pub mod widgets;

/// Possible commands.
pub mod command;

use crate::error::Result;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::Terminal;
use std::{io, panic};

/// Initializes the terminal interface.
///
/// It enables the raw mode and sets terminal properties.
pub fn tui_init<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    terminal::enable_raw_mode()?;
    ratatui::crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    panic::set_hook(Box::new(move |panic| {
        better_panic::Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic);
        std::process::exit(1);
    }));
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(())
}

/// Exits the terminal interface.
///
/// It disables the raw mode and reverts back the terminal properties.
pub fn tui_exit<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    terminal::disable_raw_mode()?;
    ratatui::crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
