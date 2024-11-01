use binsider::prelude::*;
use ratatui::{crossterm::event::KeyCode, Frame};
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut path = PathBuf::from("ls");
    if !path.exists() {
        let resolved_path = which::which(path.to_string_lossy().to_string())?;
        path = resolved_path;
    }

    let file_data = std::fs::read(&path)?;
    let bytes = file_data.as_slice();
    let file_info = FileInfo::new(
        path.to_str().expect("should be valid string"),
        Some(vec![]),
        bytes,
    )?;
    let analyzer = Analyzer::new(file_info, 15, vec![])?;

    // Create an application.
    let mut state = State::new(analyzer, None)?;
    let events = EventHandler::new(250);
    state.analyzer.extract_strings(events.sender.clone());

    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                render(&mut state, frame);
            })
            .expect("failed to draw frame");

        let event = events.next()?;
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
                handle_event(Event::Key(key_event), &events, &mut state)?;
            }
            Event::Restart(None) => {
                break;
            }
            Event::Restart(Some(path)) => {
                let file_data = std::fs::read(&path)?;
                let bytes = file_data.as_slice();
                let file_info = FileInfo::new(
                    path.to_str().expect("should be valid string"),
                    Some(vec![]),
                    bytes,
                )?;
                let analyzer = Analyzer::new(file_info, 15, vec![])?;
                state.analyzer = analyzer;
                state.handle_tab()?;
                state.analyzer.extract_strings(events.sender.clone());
            }
            _ => {
                handle_event(event, &events, &mut state)?;
            }
        }
    }
    events.stop();
    ratatui::restore();

    Ok(())
}
