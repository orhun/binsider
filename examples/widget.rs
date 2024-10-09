use std::path::PathBuf;
use ratatui::{
    crossterm::event::{KeyCode},
    Frame,
};
use binsider::prelude::Event;

fn main() {
    let mut path = PathBuf::from("ls");
    if !path.exists() {
        let resolved_path = which::which(path.to_string_lossy().to_string()).unwrap();
        path = resolved_path;
    }

    let file_data = std::fs::read(&path).unwrap();
    let bytes = file_data.as_slice();
    let file_info = binsider::file::FileInfo::new(path.to_str().unwrap(), Some(vec![]), bytes).unwrap();
    let analyzer =
        binsider::app::Analyzer::new(file_info, 15, vec![]).unwrap();

    // Create an application.
    let mut state = binsider::tui::state::State::new(analyzer).unwrap();
    let events = binsider::tui::event::EventHandler::new(250);
    state.analyzer.extract_strings(events.sender.clone());

    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                binsider::tui::ui::render(&mut state, frame);
            })
            .expect("failed to draw frame");

        let event = events.next().unwrap();
        match event {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
                binsider::handle_event(Event::Key(key_event), &events, &mut state).unwrap();
            }
            Event::Restart(path) => {
                let path = path.unwrap();
                let file_data = std::fs::read(&path).unwrap();
                let bytes = file_data.as_slice();
                let file_info = binsider::file::FileInfo::new(path.to_str().unwrap(), Some(vec![]), bytes).unwrap();
                let analyzer =
                    binsider::app::Analyzer::new(file_info, 15, vec![]).unwrap();

                state.change_analyzer(analyzer);
                state.handle_tab().unwrap();
                state.analyzer.extract_strings(events.sender.clone());
            }
            _ => {
                binsider::handle_event(event, &events, &mut state).unwrap();
            }
        }
    }
    events.stop();
    ratatui::restore();
}