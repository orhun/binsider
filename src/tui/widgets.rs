use ratatui::widgets::TableState as State;

/// List widget with TUI controlled states.
#[derive(Debug)]
pub struct SelectableList<T> {
    /// List items.
    pub items: Vec<T>,
    /// State that can be modified by TUI.
    pub state: State,
}

impl<T> Default for SelectableList<T> {
    fn default() -> Self {
        Self::with_items(Vec::new())
    }
}

impl<T> SelectableList<T> {
    /// Constructs a new instance of `SelectableList`.
    pub fn new(items: Vec<T>, mut state: State) -> SelectableList<T> {
        state.select(Some(0));
        Self { items, state }
    }

    /// Construct a new `SelectableList` with given items.
    pub fn with_items(items: Vec<T>) -> SelectableList<T> {
        Self::new(items, State::default())
    }

    /// Returns the selected item.
    pub fn selected(&self) -> Option<&T> {
        self.items.get(self.state.selected()?)
    }

    /// Selects the next item.
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Selects the previous item.
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selectable_list() {
        let mut list = SelectableList::with_items(vec!["data1", "data2", "data3"]);
        list.state.select(Some(1));
        assert_eq!(Some(&"data2"), list.selected());
        list.next();
        assert_eq!(Some(2), list.state.selected());
        list.previous();
        assert_eq!(Some(1), list.state.selected());

        let mut list = SelectableList::<()>::default();
        list.state.select(None);
        list.next();
        list.state.select(None);
        list.previous();
        assert_eq!(Some(0), list.state.selected());
    }
}
