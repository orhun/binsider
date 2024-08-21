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
    /// Exit application.
    Exit,
    /// Do nothing.
    Nothing,
}

impl From<KeyEvent> for Command {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Right | KeyCode::Char('l') => Self::Next(ScrollType::Table, 1),
            KeyCode::Left | KeyCode::Char('h') => Self::Previous(ScrollType::Table, 1),
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
        if key_event.code == KeyCode::Char('q')
            || key_event.code == KeyCode::Esc
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
    /// Warn.
    Warn(String),
    /// Cancel hexdump.
    Cancel,
    /// Exit application.
    Exit,
}

impl HexdumpCommand {
    /// Parses the event.
    pub fn parse(key_event: KeyEvent, is_read_only: bool) -> Self {
        if key_event.code == KeyCode::Char('q') {
            Self::Exit
        } else if key_event.code == KeyCode::Tab {
            Self::Cancel
        } else if key_event.code == KeyCode::Char('s') {
            if is_read_only {
                Self::Warn(String::from("file is read-only"))
            } else {
                Self::Handle(Event::Key(KeyEvent::new(
                    KeyCode::Char('s'),
                    KeyModifiers::CONTROL,
                )))
            }
        } else if key_event.code == KeyCode::Char('g') {
            Self::Handle(Event::Key(KeyEvent::new(
                KeyCode::Char('j'),
                KeyModifiers::CONTROL,
            )))
        } else if key_event.code == KeyCode::Char('n') {
            Self::Handle(Event::Key(KeyEvent::new(
                KeyCode::Char('e'),
                KeyModifiers::CONTROL,
            )))
        } else {
            Self::Handle(Event::Key(key_event))
        }
    }
}
