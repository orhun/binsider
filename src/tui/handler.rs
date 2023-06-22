use crate::error::Result;
use crate::tui::state::State;
use crate::tui::ui::{ELF_INFO_TABS, MAIN_TABS};
use crate::tui::widgets::SelectableList;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, state: &mut State) -> Result<()> {
    match key_event.code {
        // Next tab.
        KeyCode::Right | KeyCode::Char('l') => {
            state.tab_index = (state.tab_index + 1) % MAIN_TABS.len();
        }
        // Previous tab.
        KeyCode::Left | KeyCode::Char('h') => {
            if state.tab_index > 0 {
                state.tab_index -= 1;
            } else {
                state.tab_index = MAIN_TABS.len() - 1;
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
        KeyCode::Tab => {
            state.info_index = (state.info_index + 1) % ELF_INFO_TABS.len();
            state.list = SelectableList::with_items(
                state
                    .analyzer
                    .elf
                    .info(&ELF_INFO_TABS[state.info_index])
                    .items(),
            );
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
