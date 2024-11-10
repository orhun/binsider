use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind,
};
use tui_input::Input;

/// Possible scroll areas.
#[derive(Debug, PartialEq, Eq)]
pub enum ScrollType {
    /// Main application tabs.
    Tab,
    /// Inner tables.
    Table,
    /// Main list.
    List,
    /// Block.
    Block,
}

/// Application command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Open repository URL.
    OpenRepo,
    /// Show details.
    ShowDetails,
    /// Next.
    Next(ScrollType, usize),
    /// Previous.
    Previous(ScrollType, usize),
    /// Go to top.
    Top,
    /// Go to bottom.
    Bottom,
    /// Increment value.
    Increment,
    /// Decrement value.
    Decrement,
    /// Input command.
    Input(InputCommand),
    /// Hexdump command.
    Hexdump(HexdumpCommand),
    /// Trace system calls.
    TraceCalls,
    /// Sort items.
    Sort,
    /// Exit application.
    Exit,
    /// Do nothing.
    Nothing,
    /// Change data to human readable format
    HumanReadable,
}

impl From<KeyEvent> for Command {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Right | KeyCode::Char('l') => Self::Next(ScrollType::Table, 1),
            KeyCode::Left | KeyCode::Char('h') => Self::Previous(ScrollType::Table, 1),
            KeyCode::Char('n') => Self::Next(ScrollType::Block, 1),
            KeyCode::Char('p') => Self::Previous(ScrollType::Block, 1),
            KeyCode::Down | KeyCode::Char('j') => Self::Next(ScrollType::List, 1),
            KeyCode::Up | KeyCode::Char('k') => Self::Previous(ScrollType::List, 1),
            KeyCode::PageDown => Self::Next(ScrollType::List, 5),
            KeyCode::PageUp => Self::Previous(ScrollType::List, 5),
            KeyCode::Char('d') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Self::Next(ScrollType::List, 5)
                } else {
                    Self::Nothing
                }
            }
            KeyCode::Char('u') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Self::Previous(ScrollType::List, 5)
                } else {
                    Self::Nothing
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => Self::Exit,
            KeyCode::Tab => Self::Next(ScrollType::Tab, 1),
            KeyCode::BackTab => Self::Previous(ScrollType::Tab, 1),
            KeyCode::Char('t') | KeyCode::Home => Self::Top,
            KeyCode::Char('b') | KeyCode::End => Self::Bottom,
            KeyCode::Char('+') => Self::Increment,
            KeyCode::Char('-') => Self::Decrement,
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Self::Exit
                } else {
                    Self::Nothing
                }
            }
            KeyCode::Char('/') => Self::Input(InputCommand::Enter),
            KeyCode::Char('f') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Self::Input(InputCommand::Enter)
                } else {
                    Self::Nothing
                }
            }
            KeyCode::Backspace => Self::Input(InputCommand::Resume(Event::Key(key_event))),
            KeyCode::Enter => Self::ShowDetails,
            KeyCode::Char('o') => Self::OpenRepo,
            KeyCode::Char('r') => Self::TraceCalls,
            KeyCode::Char('s') => Self::HumanReadable,
            KeyCode::Char('S') => Self::Sort,
            _ => Self::Nothing,
        }
    }
}

impl From<MouseEvent> for Command {
    fn from(mouse_event: MouseEvent) -> Self {
        match mouse_event.kind {
            MouseEventKind::ScrollDown => Self::Next(ScrollType::List, 1),
            MouseEventKind::ScrollUp => Self::Previous(ScrollType::List, 1),
            _ => Self::Nothing,
        }
    }
}

/// Input mode command.
#[derive(Debug, PartialEq, Eq)]
pub enum InputCommand {
    /// Handle input.
    Handle(Event),
    /// Enter input mode.
    Enter,
    /// Confirm input.
    Confirm,
    /// Resume input.
    Resume(Event),
    /// Exit input mode
    Exit,
}

impl InputCommand {
    /// Parses the event.
    pub fn parse(key_event: KeyEvent, input: &Input) -> Self {
        if key_event.code == KeyCode::Esc
            || (key_event.code == KeyCode::Backspace && input.value().is_empty())
        {
            Self::Exit
        } else if key_event.code == KeyCode::Enter {
            Self::Confirm
        } else {
            Self::Handle(Event::Key(key_event))
        }
    }
}

/// Hexdump command.
#[derive(Debug, PartialEq, Eq)]
pub enum HexdumpCommand {
    /// Handle hexdump event.
    Handle(Event),
    /// Handle hexdump event with a custom key.
    HandleCustom(Event, Event),
    /// Warn.
    Warn(String, Event),
    /// Cancel hexdump and move to the next tab.
    CancelNext,
    /// Cancel hexdump and move to the previous tab.
    CancelPrevious,
    /// Exit application.
    Exit,
}

impl HexdumpCommand {
    /// Parses the event.
    pub fn parse(key_event: KeyEvent, is_read_only: bool) -> Self {
        match key_event.code {
            KeyCode::Char('q') => Self::Exit,
            KeyCode::Tab => Self::CancelNext,
            KeyCode::BackTab => Self::CancelPrevious,
            KeyCode::Char('s') => {
                if is_read_only {
                    Self::Warn(String::from("file is read-only"), Event::Key(key_event))
                } else {
                    Self::HandleCustom(
                        Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL)),
                        Event::Key(key_event),
                    )
                }
            }
            KeyCode::Char('g') => Self::HandleCustom(
                Event::Key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL)),
                Event::Key(key_event),
            ),
            KeyCode::Char('n') => Self::HandleCustom(
                Event::Key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL)),
                Event::Key(key_event),
            ),
            _ => Self::Handle(Event::Key(key_event)),
        }
    }
}
