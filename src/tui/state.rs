use std::path::PathBuf;
use std::sync::mpsc;

use crate::error::{Error, Result};
use crate::prelude::Analyzer;
use crate::tui::command::*;
use crate::tui::event::Event;
use crate::tui::ui::{Tab, ELF_INFO_TABS, MAIN_TABS};
use crate::tui::widgets::SelectableList;
use ansi_to_tui::IntoText;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

/// Application state.
#[derive(Debug)]
pub struct State<'a> {
    /// Is the application running?
    pub running: bool,
    /// Binary analyzer.
    pub analyzer: Analyzer<'a>,
    /// Selected tab.
    pub tab: Tab,
    /// Elf info.
    pub info_index: usize,
    /// Scroll index.
    pub scroll_index: usize,
    /// List items.
    pub list: SelectableList<Vec<String>>,
    /// Show heh.
    pub show_heh: bool,
    /// Show details.
    pub show_details: bool,
    /// Input.
    pub input: Input,
    /// Enable input.
    pub input_mode: bool,
    /// Strings call completed.
    pub strings_loaded: bool,
    /// System calls completed.
    pub system_calls_loaded: bool,
}

impl<'a> State<'a> {
    /// Constructs a new instance of [`State`].
    pub fn new(analyzer: Analyzer<'a>) -> Result<Self> {
        let mut state = Self {
            running: true,
            tab: Tab::default(),
            info_index: 0,
            scroll_index: 0,
            list: SelectableList::default(),
            analyzer,
            show_heh: false,
            show_details: false,
            input: Input::default(),
            input_mode: false,
            strings_loaded: false,
            system_calls_loaded: false,
        };
        state.handle_tab()?;
        Ok(state)
    }

    /// Runs a command and updates the state.
    pub fn run_command(
        &mut self,
        command: Command,
        event_sender: mpsc::Sender<Event>,
    ) -> Result<()> {
        match command {
            Command::Input(command) => {
                match command {
                    InputCommand::Handle(event) => {
                        self.input.handle_event(&event);
                    }
                    InputCommand::Enter => {
                        self.input_mode = true;
                    }
                    InputCommand::Confirm => {
                        self.input_mode = false;
                    }
                    InputCommand::Resume(event) => {
                        if !self.input.value().is_empty() {
                            self.input_mode = true;
                            self.input.handle_event(&event);
                        }
                    }
                    InputCommand::Exit => {
                        self.input = Input::default();
                        self.input_mode = false;
                    }
                }
                self.handle_tab()?;
            }
            Command::Hexdump(command) => match command {
                HexdumpCommand::Handle(event) => {
                    self.analyzer
                        .heh
                        .handle_input(&event)
                        .map_err(|e| Error::HexdumpError(e.to_string()))?;
                }
                HexdumpCommand::Warn(message) => {
                    self.analyzer.heh.labels.notification = message;
                }
                HexdumpCommand::Cancel => {
                    self.tab = ((self.tab as usize + 1) % MAIN_TABS.len()).into();
                    self.handle_tab()?;
                }
                HexdumpCommand::Exit => {
                    self.running = false;
                }
            },
            Command::ShowDetails => {
                if self.tab == Tab::General {
                    event_sender
                        .send(Event::Restart(
                            self.list.selected().map(|v| PathBuf::from(v[1].clone())),
                        ))
                        .expect("failed to send trace event");
                    return Ok(());
                } else if self.tab == Tab::DynamicAnalysis && !self.system_calls_loaded {
                    event_sender
                        .send(Event::Trace)
                        .expect("failed to send trace event");
                    return Ok(());
                } else {
                    self.show_details = !self.show_details;
                }
            }
            Command::OpenRepo => {
                if self.tab == Tab::General {
                    webbrowser::open(env!("CARGO_PKG_REPOSITORY"))?;
                }
            }
            Command::TraceCalls => {
                event_sender
                    .send(Event::Trace)
                    .expect("failed to send trace event");
            }
            Command::Next(scroll_type, amount) => match scroll_type {
                ScrollType::Tab => {
                    self.tab = (((self.tab as usize).checked_add(amount).unwrap_or_default())
                        % MAIN_TABS.len())
                    .into();
                    self.handle_tab()?;
                }
                ScrollType::Table => {
                    if self.tab == Tab::StaticAnalysis {
                        self.info_index = (self.info_index.checked_add(amount).unwrap_or_default())
                            % ELF_INFO_TABS.len();
                        self.handle_tab()?;
                    }
                }
                ScrollType::List => {
                    if self.tab == Tab::DynamicAnalysis {
                        self.scroll_index = self.scroll_index.saturating_add(amount);
                    } else {
                        self.list.next(amount)
                    }
                }
            },
            Command::Previous(scroll_type, amount) => match scroll_type {
                ScrollType::Tab => {
                    unimplemented!()
                }
                ScrollType::Table => {
                    if self.tab == Tab::StaticAnalysis {
                        self.info_index = self
                            .info_index
                            .checked_sub(amount)
                            .unwrap_or(ELF_INFO_TABS.len() - 1);
                        self.handle_tab()?;
                    }
                }
                ScrollType::List => {
                    if self.tab == Tab::DynamicAnalysis {
                        self.scroll_index = self.scroll_index.saturating_sub(amount);
                    } else {
                        self.list.previous(amount)
                    }
                }
            },
            Command::Top => {
                if self.tab == Tab::DynamicAnalysis {
                    self.scroll_index = 0;
                } else {
                    self.list.first();
                }
            }
            Command::Bottom => {
                if self.tab == Tab::DynamicAnalysis {
                    self.scroll_index = self
                        .analyzer
                        .tracer
                        .syscalls
                        .into_text()
                        .unwrap_or_default()
                        .lines
                        .len();
                } else {
                    self.list.last();
                }
            }
            Command::Increment => {
                if self.tab == Tab::Strings {
                    self.analyzer.strings_len = self
                        .analyzer
                        .strings_len
                        .checked_add(1)
                        .unwrap_or(self.analyzer.strings_len);
                    self.strings_loaded = false;
                    self.analyzer.extract_strings(event_sender.clone());
                }
            }
            Command::Decrement => {
                if self.tab == Tab::Strings {
                    self.analyzer.strings_len =
                        self.analyzer.strings_len.checked_sub(1).unwrap_or_default();
                    self.strings_loaded = false;
                    self.analyzer.extract_strings(event_sender.clone());
                }
            }
            Command::Exit => {
                if self.show_details {
                    self.show_details = false;
                } else {
                    self.running = false;
                }
            }
            Command::Nothing => {}
        }
        Ok(())
    }

    /// Update the state based on selected tab.
    pub fn handle_tab(&mut self) -> Result<()> {
        self.show_heh = false;
        match self.tab {
            Tab::General => {
                self.list = SelectableList::with_items(
                    self.analyzer
                        .dependencies
                        .libraries
                        .clone()
                        .into_iter()
                        .map(|(name, lib)| {
                            vec![
                                name.to_string(),
                                lib.realpath
                                    .unwrap_or(lib.path)
                                    .to_string_lossy()
                                    .to_string(),
                            ]
                        })
                        .collect(),
                );
            }
            Tab::StaticAnalysis => {
                self.list = SelectableList::with_items(
                    self.analyzer
                        .elf
                        .info(&ELF_INFO_TABS[self.info_index])
                        .items()
                        .into_iter()
                        .filter(|items| {
                            self.input.value().is_empty()
                                || items.iter().any(|item| {
                                    item.to_lowercase()
                                        .contains(&self.input.value().to_lowercase())
                                })
                        })
                        .collect(),
                );
            }
            Tab::DynamicAnalysis => {
                self.analyzer.system_calls = self
                    .analyzer
                    .tracer
                    .syscalls
                    .into_text()
                    .unwrap_or_else(|_| "ANSI error occurred".into())
                    .lines
                    .into_iter()
                    .filter(|line| {
                        self.input.value().is_empty()
                            || line
                                .clone()
                                .reset_style()
                                .to_string()
                                .to_lowercase()
                                .contains(&self.input.value().to_lowercase())
                    })
                    .collect();
            }
            Tab::Strings => {
                self.list = SelectableList::with_items(
                    self.analyzer
                        .strings
                        .clone()
                        .unwrap_or_default()
                        .iter()
                        .map(|(v, i)| vec![v.to_string(), i.to_string()])
                        .filter(|items| {
                            self.input.value().is_empty()
                                || items.iter().any(|item| {
                                    item.to_lowercase()
                                        .contains(&self.input.value().to_lowercase())
                                })
                        })
                        .collect(),
                )
            }
            Tab::Hexdump => {
                self.show_heh = true;
            }
        }
        Ok(())
    }

    /// Returns the key bindings.
    pub fn get_key_bindings(&self) -> Vec<(&'a str, &'a str)> {
        match self.tab {
            Tab::General => {
                vec![
                    ("Enter", "Analyze library"),
                    ("o", "Visit Repository"),
                    ("Tab", "Next"),
                    ("q", "Quit"),
                ]
            }
            Tab::StaticAnalysis => vec![
                ("Enter", "Details"),
                ("/", "Search"),
                ("h/j/k/l", "Scroll"),
                ("Tab", "Next"),
                ("q", "Quit"),
            ],
            Tab::DynamicAnalysis => {
                if self.system_calls_loaded {
                    vec![
                        ("Enter", "Details"),
                        ("r", "Re-run"),
                        ("/", "Search"),
                        ("h/j/k/l", "Scroll"),
                        ("Tab", "Next"),
                        ("q", "Quit"),
                    ]
                } else {
                    vec![
                        ("Enter", "Run"),
                        ("/", "Search"),
                        ("h/j/k/l", "Scroll"),
                        ("Tab", "Next"),
                        ("q", "Quit"),
                    ]
                }
            }
            Tab::Strings => vec![
                ("Enter", "Details"),
                ("+", "Increment"),
                ("-", "Decrement"),
                ("/", "Search"),
                ("h/j/k/l", "Scroll"),
                ("Tab", "Next"),
                ("q", "Quit"),
            ],
            Tab::Hexdump => vec![
                ("s", "Save"),
                ("g", "Jump"),
                ("/", "Search"),
                ("n", "Endianness"),
                ("h/j/k/l", "Scroll"),
                ("Tab", "Next"),
                ("q", "Quit"),
            ],
        }
    }
}
