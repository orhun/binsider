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

    /// Selects the first item.
    pub fn first(&mut self) {
        self.state.select(Some(0));
    }

    /// Selects the last item.
    pub fn last(&mut self) {
        self.state.select(Some(self.items.len().saturating_sub(1)));
    }

    /// Selects the next item.
    pub fn next(&mut self, amount: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i.saturating_add(amount) >= self.items.len() {
                    0
                } else {
                    i.saturating_add(amount)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Selects the previous item.
    pub fn previous(&mut self, amount: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i.saturating_sub(amount)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Restores a previously selected index, clamping it to the current items.
    ///
    /// Useful after replacing `items` (e.g. via `with_items`) with data that
    /// did not actually reorder or resize the list, where resetting the
    /// selection back to the top would otherwise be surprising.
    pub fn reselect(&mut self, index: Option<usize>) {
        if let Some(index) = index {
            if !self.items.is_empty() {
                self.state.select(Some(index.min(self.items.len() - 1)));
            }
        }
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
        list.next(1);
        assert_eq!(Some(2), list.state.selected());
        list.previous(1);
        assert_eq!(Some(1), list.state.selected());

        let mut list = SelectableList::<()>::default();
        list.state.select(None);
        list.next(1);
        list.state.select(None);
        list.previous(1);
        assert_eq!(Some(0), list.state.selected());
    }

    #[test]
    fn test_reselect() {
        let mut list = SelectableList::with_items(vec!["data1", "data2", "data3"]);
        list.state.select(Some(1));
        let previous = list.state.selected();

        // Rebuilding via `with_items` resets the selection to the top.
        list = SelectableList::with_items(vec!["data1", "data2", "data3"]);
        assert_eq!(Some(0), list.state.selected());

        // `reselect` restores the previously selected index.
        list.reselect(previous);
        assert_eq!(Some(1), list.state.selected());

        // An out-of-range index is clamped to the last item.
        list.reselect(Some(10));
        assert_eq!(Some(2), list.state.selected());

        // Reselecting on an empty list is a no-op.
        let mut empty = SelectableList::<&str>::with_items(vec![]);
        empty.state.select(None);
        empty.reselect(Some(1));
        assert_eq!(None, empty.state.selected());
    }
}
