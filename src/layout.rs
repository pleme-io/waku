//! Window positioning and sizing utilities.

/// Anchor point for floating window positioning.
#[derive(Debug, Clone, Copy, Default)]
pub enum Anchor {
    /// Anchored at the top-left corner of the float.
    #[default]
    NorthWest,
    /// Anchored at the top-right corner.
    NorthEast,
    /// Anchored at the bottom-west corner.
    SouthWest,
    /// Anchored at the bottom-east corner.
    SouthEast,
    /// Centered in the editor.
    Center,
}

/// Size specification for a UI element.
#[derive(Debug, Clone, Copy)]
pub enum Size {
    /// Fixed number of rows or columns.
    Fixed(u32),
    /// Percentage of the editor width/height (0.0–1.0).
    Percent(f64),
    /// Fit to content (compute at render time).
    Fit,
}

impl Size {
    /// Resolve to a concrete value given the available space.
    #[must_use]
    pub fn resolve(self, available: u32) -> u32 {
        match self {
            Self::Fixed(n) => n.min(available),
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            Self::Percent(p) => ((f64::from(available) * p.clamp(0.0, 1.0)) as u32).max(1),
            Self::Fit => available,
        }
    }
}

/// Layout specification for a floating window.
#[derive(Debug, Clone)]
pub struct FloatLayout {
    pub width: Size,
    pub height: Size,
    pub anchor: Anchor,
    /// Row offset from anchor (can be negative).
    pub row_offset: i32,
    /// Column offset from anchor (can be negative).
    pub col_offset: i32,
}

impl Default for FloatLayout {
    fn default() -> Self {
        Self {
            width: Size::Percent(0.6),
            height: Size::Percent(0.6),
            anchor: Anchor::Center,
            row_offset: 0,
            col_offset: 0,
        }
    }
}

impl FloatLayout {
    /// Compute concrete (row, col, width, height) for a given editor size.
    #[must_use]
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    pub fn resolve(&self, editor_width: u32, editor_height: u32) -> (u32, u32, u32, u32) {
        let w = self.width.resolve(editor_width);
        let h = self.height.resolve(editor_height);

        let (base_row, base_col) = match self.anchor {
            Anchor::NorthWest => (0i32, 0i32),
            Anchor::NorthEast => (0, editor_width as i32 - w as i32),
            Anchor::SouthWest => (editor_height as i32 - h as i32, 0),
            Anchor::SouthEast => (
                editor_height as i32 - h as i32,
                editor_width as i32 - w as i32,
            ),
            Anchor::Center => (
                (editor_height as i32 - h as i32) / 2,
                (editor_width as i32 - w as i32) / 2,
            ),
        };

        let row = (base_row + self.row_offset).max(0) as u32;
        let col = (base_col + self.col_offset).max(0) as u32;

        (row, col, w, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_size_resolve() {
        assert_eq!(Size::Fixed(10).resolve(100), 10);
        assert_eq!(Size::Fixed(200).resolve(100), 100); // clamped
    }

    #[test]
    fn percent_size_resolve() {
        assert_eq!(Size::Percent(0.5).resolve(100), 50);
        assert_eq!(Size::Percent(1.0).resolve(80), 80);
        assert_eq!(Size::Percent(0.0).resolve(80), 1); // at least 1
    }

    #[test]
    fn center_layout() {
        let layout = FloatLayout::default();
        let (row, col, w, h) = layout.resolve(100, 50);
        assert_eq!(w, 60); // 60% of 100
        assert_eq!(h, 30); // 60% of 50
        assert_eq!(row, 10); // (50-30)/2
        assert_eq!(col, 20); // (100-60)/2
    }
}
