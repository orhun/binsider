//! **binsider** - Analyze ELF binaries like a boss 😼🕵️‍♂️
//!
//! See the [documentation](https://binsider.dev) for more information.

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
#[cfg(feature = "dynamic-analysis")]
pub mod tracer;

/// File information.
pub mod file;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use args::Args;
use file::FileInfo;
use prelude::*;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{env, fs, io};
use tui::{state::State, ui::Tab, Tui};

/// Runs binsider.
pub fn run(mut args: Args) -> Result<()> {
    if args.files.is_empty() {
        args.files.push(env::current_exe()?);
    }
    let mut path = args.files[args.files.len() - 1].clone();
    if !path.exists() {
        let resolved_path = which::which(path.to_string_lossy().to_string())?;
        if let Some(file) = args.files.iter_mut().find(|f| **f == path) {
            *file = resolved_path.clone();
        }
        path = resolved_path;
    }
    let file_data = fs::read(&path)?;
    let bytes = file_data.as_slice();
    let file_info = FileInfo::new(path.to_str().unwrap_or_default(), bytes)?;
    let analyzer = Analyzer::new(file_info, args.min_strings_len, args.files.clone())?;
    start_tui(analyzer, args)
}

/// Starts the terminal user interface.
pub fn start_tui(analyzer: Analyzer, args: Args) -> Result<()> {
    // Create an application.
    let mut state = State::new(analyzer)?;

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
            Event::Tick => {}
            Event::Key(key_event) => {
                let command = if state.input_mode {
                    Command::Input(InputCommand::parse(key_event, &state.input))
                } else if state.show_heh {
                    Command::Hexdump(HexdumpCommand::parse(
                        key_event,
                        state.analyzer.file.is_read_only,
                    ))
                } else {
                    Command::from(key_event)
                };
                state.run_command(command, tui.events.sender.clone())?;
            }
            Event::Mouse(mouse_event) => {
                state.run_command(Command::from(mouse_event), tui.events.sender.clone())?;
            }
            Event::Resize(_, _) => {}
            Event::FileStrings(strings) => {
                state.strings_loaded = true;
                state.analyzer.strings = Some(strings?.into_iter().map(|(v, l)| (l, v)).collect());
                if state.tab == Tab::Strings {
                    state.handle_tab()?;
                }
            }
            #[cfg(feature = "dynamic-analysis")]
            Event::Trace => {
                state.system_calls_loaded = false;
                tui.toggle_pause()?;
                tracer::trace_syscalls(state.analyzer.file.path, tui.events.sender.clone());
            }
            #[cfg(feature = "dynamic-analysis")]
            Event::TraceResult(syscalls) => {
                state.analyzer.tracer = match syscalls {
                    Ok(v) => v,
                    Err(e) => TraceData {
                        syscalls: console::style(e).red().to_string().as_bytes().to_vec(),
                        ..Default::default()
                    },
                };
                state.system_calls_loaded = true;
                state.dynamic_scroll_index = 0;
                tui.toggle_pause()?;
                state.handle_tab()?;
            }
            #[cfg(not(feature = "dynamic-analysis"))]
            Event::Trace | Event::TraceResult(_) => {}
            Event::Restart(path) => {
                let mut args = args.clone();
                match path {
                    Some(path) => {
                        args.files.push(path);
                    }
                    None => {
                        args.files.pop();
                    }
                }
                if !args.files.is_empty() {
                    tui.exit()?;
                    state.running = false;
                    run(args)?;
                }
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
