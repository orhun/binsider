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
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use event::EventHandler;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use state::State;
use std::sync::atomic::Ordering;
use std::{io, panic};

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
    /// Is the interface paused?
    pub paused: bool,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self {
            terminal,
            events,
            paused: false,
        }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            better_panic::Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .create_panic_handler()(panic);
            std::process::exit(1);
        }));
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self, app: &mut State) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    /// Toggles the [`paused`] state of interface.
    ///
    /// It disables the key input and exits the
    /// terminal interface on pause (and vice-versa).
    ///
    /// [`paused`]: Tui::paused
    pub fn toggle_pause(&mut self) -> Result<()> {
        self.paused = !self.paused;
        if self.paused {
            Self::reset()?;
        } else {
            self.init()?;
        }
        self.events
            .key_input_disabled
            .store(self.paused, Ordering::Relaxed);
        Ok(())
    }

    /// Reset the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Terminal::new(CrosstermBackend::new(io::stderr()))?.show_cursor()?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
