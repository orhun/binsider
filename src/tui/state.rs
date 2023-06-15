use elf::segment::ProgramHeader;

use crate::prelude::Analyzer;

use super::widgets::SelectableList;

/// Application state.
#[derive(Debug)]
pub struct State<'a> {
    /// Binary analyzer.
    pub analyzer: Analyzer<'a>,
    /// Is the application running?
    pub running: bool,
    /// Index of the selected tab.
    pub tab_index: usize,
    /// List items.
    pub list: SelectableList<ProgramHeader>,
}

impl<'a> State<'a> {
    /// Constructs a new instance of [`State`].
    pub fn new(analyzer: Analyzer<'a>) -> Self {
        Self {
            list: SelectableList::with_items(analyzer.elf.program_headers.clone()),
            running: true,
            tab_index: 0,
            analyzer,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Sets [`running`] to `false` to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
