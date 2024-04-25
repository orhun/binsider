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

/// System call tracer.
pub mod tracer;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use app::Analyzer;
use args::Args;
use error::Result;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{env, fs, io};
use tracer::TraceData;
use tui::command::{Command, HexdumpCommand, InputCommand};
use tui::event::{Event, EventHandler};
use tui::state::State;
use tui::ui::Tab;
use tui::Tui;

/// Runs binsider.
pub fn run(args: Args) -> Result<()> {
    let mut file = args.file.clone().unwrap_or(env::current_exe()?);
    if !file.exists() {
        file = which::which(file.to_string_lossy().to_string())?;
    }
    let file_data = fs::read(&file)?;
    let bytes = file_data.as_slice();
    let analyzer = Analyzer::new(
        file.to_str().unwrap_or_default(),
        bytes,
        args.min_strings_len,
    )?;
    start_tui(analyzer)
}

/// Starts the terminal user interface.
pub fn start_tui(analyzer: Analyzer) -> Result<()> {
    // Create an application.
    let mut state = State::new(analyzer);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    state.analyzer.extract_strings(events.sender.clone());
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while state.running {
        // Render the user interface.
        tui.draw(&mut state)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => state.tick(),
            Event::Key(key_event) => {
                let command = if state.input_mode {
                    Command::Input(InputCommand::parse(key_event, &state.input))
                } else if state.show_heh {
                    Command::Hexdump(HexdumpCommand::parse(
                        key_event,
                        state.analyzer.is_read_only,
                    ))
                } else {
                    Command::from(key_event)
                };
                state.run_command(command, tui.events.sender.clone())?;
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::FileStrings(strings) => {
                state.analyzer.strings = Some(strings?.into_iter().map(|(v, l)| (l, v)).collect());
                if state.tab == Tab::Strings {
                    state.handle_tab()?;
                }
            }
            Event::Trace => {
                tui.toggle_pause()?;
                tracer::trace_syscalls(state.analyzer.path, tui.events.sender.clone());
            }
            Event::TraceResult(syscalls) => {
                state.analyzer.tracer = match syscalls {
                    Ok(v) => v,
                    Err(e) => TraceData {
                        syscalls: console::style(e).red().to_string().as_bytes().to_vec(),
                        ..Default::default()
                    },
                };
                tui.toggle_pause()?;
                state.handle_tab()?;
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
