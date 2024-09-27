//! A simple demo of how to use the binsider as a library.

use std::{env, fs, path::PathBuf, sync::mpsc, time::Duration};

use binsider::{prelude::*, tui::ui::Tab};

use ratatui::{
    crossterm::event::{self, Event as CrosstermEvent, KeyCode},
    Frame,
};

fn main() -> Result<()> {
    // Create an analyzer.
    let path = PathBuf::from(env::args().next().expect("no file given"));
    let file_data = fs::read(&path)?;
    let file_info = FileInfo::new(
        path.to_str().unwrap_or_default(),
        None,
        file_data.as_slice(),
    )?;
    let analyzer = Analyzer::new(file_info, 15, vec![])?;
    let mut state = State::new(analyzer)?;
    let (sender, receiver) = mpsc::channel();
    state.analyzer.extract_strings(sender.clone());

    let mut terminal = ratatui::init();
    loop {
        // Render the UI.
        terminal.draw(|frame: &mut Frame| {
            binsider::tui::ui::render(&mut state, frame);
        })?;

        // Handle terminal events.
        if event::poll(Duration::from_millis(16))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                let command = Command::from(key);
                state.run_command(command, sender.clone())?;
            }
        }

        // Handle binsider events.
        if let Some(event) = receiver.try_recv().ok() {
            match event {
                Event::FileStrings(strings) => {
                    state.strings_loaded = true;
                    state.analyzer.strings =
                        Some(strings?.into_iter().map(|(v, l)| (l, v)).collect());
                    if state.tab == Tab::Strings {
                        state.handle_tab()?;
                    }
                }
                _ => {}
            }
        }
    }
    ratatui::restore();

    Ok(())
}
