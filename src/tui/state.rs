use crate::prelude::Analyzer;

/// Application state.
#[derive(Debug)]
pub struct State<'a> {
    /// Binary analyzer.
    pub analyzer: Analyzer<'a>,
    /// Is the application running?
    pub running: bool,
    /// Index of the selected tab.
    pub tab_index: usize,
}

impl<'a> State<'a> {
    /// Constructs a new instance of [`State`].
    pub fn new(analyzer: Analyzer<'a>) -> Self {
        Self {
            analyzer,
            running: true,
            tab_index: 0,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Sets [`running`] to `false` to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
