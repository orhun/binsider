use crate::elf::Info;
use crate::prelude::Analyzer;
use crate::tui::widgets::SelectableList;

/// Application state.
#[derive(Debug)]
pub struct State<'a> {
    /// Is the application running?
    pub running: bool,
    /// Binary analyzer.
    pub analyzer: Analyzer<'a>,
    /// Index of the selected tab.
    pub tab_index: usize,
    /// Elf info.
    pub info_index: usize,
    /// List items.
    pub list: SelectableList<Vec<String>>,
}

impl<'a> State<'a> {
    /// Constructs a new instance of [`State`].
    pub fn new(analyzer: Analyzer<'a>) -> Self {
        Self {
            running: true,
            tab_index: 0,
            info_index: 0,
            list: SelectableList::with_items(analyzer.elf.info(&Info::ProgramHeaders).items()),
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
