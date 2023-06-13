/// Application state.
#[derive(Debug)]
pub struct State {
    /// Is the application running?
    pub running: bool,
    /// Index of the selected tab.
    pub tab_index: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            running: true,
            tab_index: 0,
        }
    }
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Sets [`running`] to `false` to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
