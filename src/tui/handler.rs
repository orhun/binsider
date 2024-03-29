use crate::error::Result;
use crate::tui::event::Event;
use crate::tui::state::State;
use crate::tui::ui::{Tab, ELF_INFO_TABS, MAIN_TABS};
use crate::tui::widgets::SelectableList;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    state: &mut State,
    event_sender: mpsc::Sender<Event>,
) -> Result<()> {
    match key_event.code {
        // Next info.
        KeyCode::Right | KeyCode::Char('l') => {
            if state.tab == Tab::StaticAnalysis {
                state.info_index = (state.info_index + 1) % ELF_INFO_TABS.len();
                state.list = SelectableList::with_items(
                    state
                        .analyzer
                        .elf
                        .info(&ELF_INFO_TABS[state.info_index])
                        .items(),
                );
            }
        }
        // Previous info.
        KeyCode::Left | KeyCode::Char('h') => {
            if state.tab == Tab::StaticAnalysis {
                state.info_index = state
                    .info_index
                    .checked_sub(1)
                    .unwrap_or(ELF_INFO_TABS.len() - 1);
                state.list = SelectableList::with_items(
                    state
                        .analyzer
                        .elf
                        .info(&ELF_INFO_TABS[state.info_index])
                        .items(),
                );
            }
        }
        // Scroll down the list.
        KeyCode::Down | KeyCode::Char('j') => state.list.next(),
        // Scroll up the list.
        KeyCode::Up | KeyCode::Char('k') => state.list.previous(),
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            state.quit();
        }
        // Next tab..
        KeyCode::Tab => {
            state.tab = ((state.tab as usize + 1) % MAIN_TABS.len()).into();
            handle_tab(state)?;
        }
        // Increase string length.
        KeyCode::Char('+') => {
            if state.tab == Tab::Strings {
                state.analyzer.strings_len = state
                    .analyzer
                    .strings_len
                    .checked_add(1)
                    .unwrap_or(state.analyzer.strings_len);
                state.analyzer.extract_strings(event_sender.clone());
            }
        }
        // Decrease string length.
        KeyCode::Char('-') => {
            if state.tab == Tab::Strings {
                state.analyzer.strings_len = state
                    .analyzer
                    .strings_len
                    .checked_sub(1)
                    .unwrap_or_default();
                state.analyzer.extract_strings(event_sender.clone());
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                state.quit();
            }
        }
        _ => {}
    }
    Ok(())
}

/// Update the state based on selected tab.
pub fn handle_tab(state: &mut State) -> Result<()> {
    match state.tab {
        Tab::StaticAnalysis => {
            state.info_index = 0;
            state.list = SelectableList::with_items(
                state
                    .analyzer
                    .elf
                    .info(&ELF_INFO_TABS[state.info_index])
                    .items(),
            );
        }
        Tab::Strings => {
            state.list = SelectableList::with_items(
                state
                    .analyzer
                    .strings
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|(v, i)| vec![v.to_string(), i.to_string()])
                    .collect(),
            )
        }
        _ => {}
    }
    Ok(())
}
