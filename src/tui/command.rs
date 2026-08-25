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
    /// Exit application.
    Exit,
    /// Do nothing.
    Nothing,
    /// Change data to human readable format
    HumanReadable,
    /// Command prompt command.
    CommandPrompt(CommandPromptCommand),
}

impl Command {
    /// Parses a command typed in the command prompt (e.g. `:quit`).
    ///
    /// Returns [`None`] if the input does not match any known command.
    pub fn parse_prompt(input: &str) -> Option<Self> {
        match input.trim() {
            "q" | "quit" | "exit" => Some(Self::Exit),
            "top" => Some(Self::Top),
            "bottom" => Some(Self::Bottom),
            "next" => Some(Self::Next(ScrollType::Tab, 1)),
            "previous" | "prev" => Some(Self::Previous(ScrollType::Tab, 1)),
            "readability" => Some(Self::HumanReadable),
            "docs" | "help" => Some(Self::OpenRepo),
            "trace" => Some(Self::TraceCalls),
            _ => None,
        }
    }
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
            KeyCode::Char(':') => Self::CommandPrompt(CommandPromptCommand::Enter),
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

/// Command prompt (`:`) mode command.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandPromptCommand {
    /// Handle a key event while typing a command.
    Handle(Event),
    /// Enter command prompt mode.
    Enter,
    /// Run the typed command.
    Confirm,
    /// Exit command prompt mode.
    Exit,
}

impl CommandPromptCommand {
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
    Exit(Event),
}

impl HexdumpCommand {
    /// Parses the event.
    pub fn parse(key_event: KeyEvent, is_read_only: bool) -> Self {
        match key_event.code {
            KeyCode::Char('q') => Self::Exit(Event::Key(key_event)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enter_command_prompt() {
        assert_eq!(
            Command::from(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE)),
            Command::CommandPrompt(CommandPromptCommand::Enter)
        );
    }

    #[test]
    fn test_parse_command_prompt_command() {
        let input = Input::default().with_value(String::from("quit"));
        assert_eq!(
            CommandPromptCommand::parse(
                KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
                &input
            ),
            CommandPromptCommand::Handle(Event::Key(KeyEvent::new(
                KeyCode::Char('t'),
                KeyModifiers::NONE
            )))
        );
        assert_eq!(
            CommandPromptCommand::parse(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &input),
            CommandPromptCommand::Confirm
        );
        assert_eq!(
            CommandPromptCommand::parse(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), &input),
            CommandPromptCommand::Exit
        );
        assert_eq!(
            CommandPromptCommand::parse(
                KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
                &Input::default()
            ),
            CommandPromptCommand::Exit
        );
    }

    #[test]
    fn test_parse_prompt() {
        assert_eq!(Command::parse_prompt("quit"), Some(Command::Exit));
        assert_eq!(Command::parse_prompt("q"), Some(Command::Exit));
        assert_eq!(Command::parse_prompt("  exit  "), Some(Command::Exit));
        assert_eq!(Command::parse_prompt("top"), Some(Command::Top));
        assert_eq!(Command::parse_prompt("bottom"), Some(Command::Bottom));
        assert_eq!(
            Command::parse_prompt("next"),
            Some(Command::Next(ScrollType::Tab, 1))
        );
        assert_eq!(
            Command::parse_prompt("prev"),
            Some(Command::Previous(ScrollType::Tab, 1))
        );
        assert_eq!(
            Command::parse_prompt("readability"),
            Some(Command::HumanReadable)
        );
        assert_eq!(Command::parse_prompt("docs"), Some(Command::OpenRepo));
        assert_eq!(Command::parse_prompt("trace"), Some(Command::TraceCalls));
        assert_eq!(Command::parse_prompt("bogus"), None);
        assert_eq!(Command::parse_prompt(""), None);
    }
}
