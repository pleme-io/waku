//! Scrollable, selectable list widget.

/// An item in a selectable list.
#[derive(Debug, Clone)]
pub struct ListItem {
    /// Display text for this item.
    pub text: String,
    /// Optional icon prefix.
    pub icon: Option<String>,
    /// Optional highlight group for the text.
    pub highlight: Option<String>,
    /// Arbitrary user data attached to this item.
    pub data: Option<String>,
}

impl ListItem {
    /// Create a new list item with the given text.
    #[must_use]
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            icon: None,
            highlight: None,
            data: None,
        }
    }

    /// Set an icon prefix.
    #[must_use]
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Set a highlight group.
    #[must_use]
    pub fn highlight(mut self, hl: &str) -> Self {
        self.highlight = Some(hl.to_string());
        self
    }

    /// Attach user data.
    #[must_use]
    pub fn data(mut self, data: &str) -> Self {
        self.data = Some(data.to_string());
        self
    }

    /// Render to a display string (icon + text).
    #[must_use]
    pub fn render(&self) -> String {
        match &self.icon {
            Some(icon) => format!("{icon} {}", self.text),
            None => self.text.clone(),
        }
    }
}

/// State for a scrollable list.
#[derive(Debug)]
pub struct ListState {
    /// Items in the list.
    pub items: Vec<ListItem>,
    /// Currently selected index.
    pub selected: usize,
    /// Scroll offset (first visible item index).
    pub scroll_offset: usize,
    /// Number of visible rows.
    pub visible_rows: usize,
}

impl ListState {
    /// Create a new list state.
    #[must_use]
    pub fn new(items: Vec<ListItem>, visible_rows: usize) -> Self {
        Self {
            items,
            selected: 0,
            scroll_offset: 0,
            visible_rows,
        }
    }

    /// Move selection down.
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = (self.selected + 1).min(self.items.len() - 1);
        self.ensure_visible();
    }

    /// Move selection up.
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.ensure_visible();
    }

    /// Jump to the first item.
    pub fn select_first(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Jump to the last item.
    pub fn select_last(&mut self) {
        if !self.items.is_empty() {
            self.selected = self.items.len() - 1;
            self.ensure_visible();
        }
    }

    /// Get the currently selected item.
    #[must_use]
    pub fn selected_item(&self) -> Option<&ListItem> {
        self.items.get(self.selected)
    }

    /// Get the visible slice of items for rendering.
    #[must_use]
    pub fn visible_items(&self) -> &[ListItem] {
        let end = (self.scroll_offset + self.visible_rows).min(self.items.len());
        &self.items[self.scroll_offset..end]
    }

    /// Render visible items as display strings.
    #[must_use]
    pub fn render_lines(&self) -> Vec<String> {
        self.visible_items().iter().map(ListItem::render).collect()
    }

    fn ensure_visible(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + self.visible_rows {
            self.scroll_offset = self.selected - self.visible_rows + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_items(n: usize) -> Vec<ListItem> {
        (0..n).map(|i| ListItem::new(&format!("item {i}"))).collect()
    }

    #[test]
    fn select_next_wraps_at_end() {
        let mut state = ListState::new(make_items(3), 3);
        state.select_next();
        assert_eq!(state.selected, 1);
        state.select_next();
        assert_eq!(state.selected, 2);
        state.select_next();
        assert_eq!(state.selected, 2); // stays at end
    }

    #[test]
    fn select_prev_stops_at_zero() {
        let mut state = ListState::new(make_items(3), 3);
        state.select_prev();
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn scrolling() {
        let mut state = ListState::new(make_items(10), 3);
        // Select item 4 (beyond visible)
        for _ in 0..4 {
            state.select_next();
        }
        assert_eq!(state.selected, 4);
        assert_eq!(state.scroll_offset, 2); // scrolled to keep selected visible
    }

    #[test]
    fn render_with_icon() {
        let item = ListItem::new("file.rs").icon("");
        assert_eq!(item.render(), " file.rs");
    }
}
