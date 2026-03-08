//! Single-line input prompt.

use crate::border::BorderStyle;
use crate::layout::{Anchor, FloatLayout, Size};

/// Configuration for an input prompt.
pub struct InputPrompt {
    pub prompt: String,
    pub default_value: String,
    pub border: BorderStyle,
    pub width: Size,
}

impl InputPrompt {
    /// Create a new input prompt with the given prompt text.
    #[must_use]
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            default_value: String::new(),
            border: BorderStyle::Rounded,
            width: Size::Fixed(40),
        }
    }

    /// Set a default value.
    #[must_use]
    pub fn default_value(mut self, value: &str) -> Self {
        self.default_value = value.to_string();
        self
    }

    /// Set the border style.
    #[must_use]
    pub fn border(mut self, border: BorderStyle) -> Self {
        self.border = border;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    /// Get the layout for this input prompt (centered, single row).
    #[must_use]
    pub fn layout(&self) -> FloatLayout {
        FloatLayout {
            width: self.width,
            height: Size::Fixed(1),
            anchor: Anchor::Center,
            row_offset: 0,
            col_offset: 0,
        }
    }
}
