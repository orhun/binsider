use crate::error::{Error, Result};
use crate::tui::event::Event;
use crate::tui::state::State;
use crate::tui::ui::{Tab, ELF_INFO_TABS, MAIN_TABS};
use crate::tui::widgets::SelectableList;
use ansi_to_tui::IntoText;
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use ratatui::text::Text;
use std::sync::mpsc;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(
    key_event: KeyEvent,
    state: &mut State,
    event_sender: mpsc::Sender<Event>,
) -> Result<()> {
    if state.input_mode {
        if key_event.code == KeyCode::Char('q')
            || key_event.code == KeyCode::Esc
            || (key_event.code == KeyCode::Backspace && state.input.value().is_empty())
        {
            state.input = Input::default();
            state.input_mode = false;
        } else if key_event.code == KeyCode::Enter {
            state.input_mode = false;
        } else {
            state.input.handle_event(&CrosstermEvent::Key(key_event));
        }
        handle_tab(state)?;
        return Ok(());
    }
    if state.show_heh {
        if key_event.code == KeyCode::Char('q') {
            state.quit()
        } else if key_event.code == KeyCode::Tab {
            state.tab = ((state.tab as usize + 1) % MAIN_TABS.len()).into();
            handle_tab(state)?;
        } else if key_event.code == KeyCode::Char('s')
            && key_event.modifiers == KeyModifiers::CONTROL
            && state.analyzer.read_only
        {
            state.analyzer.heh.labels.notification = String::from("file is read-only");
        } else {
            state
                .analyzer
                .heh
                .handle_input(&CrosstermEvent::Key(key_event))
                .map_err(|e| Error::HexdumpError(e.to_string()))?;
        }
        return Ok(());
    }
    match key_event.code {
        KeyCode::Right | KeyCode::Char('l') => {
            if state.tab == Tab::StaticAnalysis {
                state.info_index = (state.info_index + 1) % ELF_INFO_TABS.len();
                handle_tab(state)?;
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if state.tab == Tab::StaticAnalysis {
                state.info_index = state
                    .info_index
                    .checked_sub(1)
                    .unwrap_or(ELF_INFO_TABS.len() - 1);
                handle_tab(state)?;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if state.tab == Tab::DynamicAnalysis {
                state.scroll_index = state.scroll_index.saturating_add(1);
            } else {
                state.list.next()
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if state.tab == Tab::DynamicAnalysis {
                state.scroll_index = state.scroll_index.saturating_sub(1);
            } else {
                state.list.previous()
            }
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            if state.show_details {
                state.show_details = false;
            } else {
                state.quit();
            }
        }
        KeyCode::Tab => {
            state.tab = ((state.tab as usize + 1) % MAIN_TABS.len()).into();
            handle_tab(state)?;
        }
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
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                state.quit();
            }
        }
        KeyCode::Char('/') => {
            state.input_mode = true;
        }
        KeyCode::Backspace => {
            if !state.input.value().is_empty() {
                state.input_mode = true;
                state.input.handle_event(&CrosstermEvent::Key(key_event));
            }
        }
        KeyCode::Enter => {
            if state.tab == Tab::DynamicAnalysis && state.analyzer.tracer.syscalls.is_empty() {
                event_sender
                    .send(Event::Trace)
                    .expect("failed to send trace event");
                return Ok(());
            } else {
                state.show_details = !state.show_details;
            }
        }
        _ => {}
    }
    state.show_details = key_event == KeyCode::Enter.into();
    Ok(())
}

/// Update the state based on selected tab.
pub fn handle_tab(state: &mut State) -> Result<()> {
    state.show_heh = false;
    match state.tab {
        Tab::StaticAnalysis => {
            state.list = SelectableList::with_items(
                state
                    .analyzer
                    .elf
                    .info(&ELF_INFO_TABS[state.info_index])
                    .items()
                    .into_iter()
                    .filter(|items| {
                        state.input.value().is_empty()
                            || items.iter().any(|item| {
                                item.to_lowercase()
                                    .contains(&state.input.value().to_lowercase())
                            })
                    })
                    .collect(),
            );
        }
        Tab::DynamicAnalysis => {
            state.system_calls = state
                .analyzer
                .tracer
                .syscalls
                .into_text()
                .unwrap_or_else(|_| Text::from("ANSI error occurred"))
                .lines
                .into_iter()
                .filter(|line| {
                    state.input.value().is_empty()
                        || line
                            .clone()
                            .reset_style()
                            .to_string()
                            .to_lowercase()
                            .contains(&state.input.value().to_lowercase())
                })
                .collect();
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
                    .filter(|items| {
                        state.input.value().is_empty()
                            || items.iter().any(|item| {
                                item.to_lowercase()
                                    .contains(&state.input.value().to_lowercase())
                            })
                    })
                    .collect(),
            )
        }
        Tab::Hexdump => {
            state.show_heh = true;
        }
    }
    Ok(())
}
