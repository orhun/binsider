use std::sync::mpsc;

use crate::elf::Info;
use crate::prelude::Analyzer;
use crate::tui::ui::Tab;
use crate::tui::widgets::SelectableList;
use ansi_to_tui::IntoText;
use ratatui::text::Line;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use super::command::{Command, HexdumpCommand, InputCommand, ScrollType};
use super::event::Event;
use super::ui::{ELF_INFO_TABS, MAIN_TABS};
use crate::error::{Error, Result};

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
    /// System calls.
    pub system_calls: Vec<Line<'a>>,
}

impl<'a> State<'a> {
    /// Constructs a new instance of [`State`].
    pub fn new(analyzer: Analyzer<'a>) -> Self {
        Self {
            running: true,
            tab: Tab::default(),
            info_index: 0,
            scroll_index: 0,
            list: SelectableList::with_items(analyzer.elf.info(&Info::ProgramHeaders).items()),
            analyzer,
            show_heh: false,
            show_details: false,
            input: Input::default(),
            input_mode: false,
            system_calls: Vec::new(),
        }
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
                if self.tab == Tab::DynamicAnalysis && self.analyzer.tracer.syscalls.is_empty() {
                    event_sender
                        .send(Event::Trace)
                        .expect("failed to send trace event");
                    return Ok(());
                } else {
                    self.show_details = !self.show_details;
                }
            }
            Command::Next(scroll_type) => match scroll_type {
                ScrollType::Tab => {
                    self.tab = ((self.tab as usize + 1) % MAIN_TABS.len()).into();
                    self.handle_tab()?;
                }
                ScrollType::Table => {
                    if self.tab == Tab::StaticAnalysis {
                        self.info_index = (self.info_index + 1) % ELF_INFO_TABS.len();
                        self.handle_tab()?;
                    }
                }
                ScrollType::List => {
                    if self.tab == Tab::DynamicAnalysis {
                        self.scroll_index = self.scroll_index.saturating_add(1);
                    } else {
                        self.list.next()
                    }
                }
            },
            Command::Previous(scroll_type) => match scroll_type {
                ScrollType::Tab => {
                    unimplemented!()
                }
                ScrollType::Table => {
                    if self.tab == Tab::StaticAnalysis {
                        self.info_index = self
                            .info_index
                            .checked_sub(1)
                            .unwrap_or(ELF_INFO_TABS.len() - 1);
                        self.handle_tab()?;
                    }
                }
                ScrollType::List => {
                    if self.tab == Tab::DynamicAnalysis {
                        self.scroll_index = self.scroll_index.saturating_sub(1);
                    } else {
                        self.list.previous()
                    }
                }
            },
            Command::Increment => {
                if self.tab == Tab::Strings {
                    self.analyzer.strings_len = self
                        .analyzer
                        .strings_len
                        .checked_add(1)
                        .unwrap_or(self.analyzer.strings_len);
                    self.analyzer.extract_strings(event_sender.clone());
                }
            }
            Command::Decrement => {
                if self.tab == Tab::Strings {
                    self.analyzer.strings_len =
                        self.analyzer.strings_len.checked_sub(1).unwrap_or_default();
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
                self.system_calls = self
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

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}
}
