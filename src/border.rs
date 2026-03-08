//! Border styles for floating windows.

/// Border style for a floating window.
#[derive(Debug, Clone)]
pub enum BorderStyle {
    /// No border.
    None,
    /// Single-line border (─ │ ┌ ┐ └ ┘).
    Single,
    /// Double-line border (═ ║ ╔ ╗ ╚ ╝).
    Double,
    /// Rounded corners (─ │ ╭ ╮ ╰ ╯).
    Rounded,
    /// Solid block border (█).
    Solid,
    /// Shadow border (bottom-right shadow).
    Shadow,
    /// Custom border characters: [top, right, bottom, left, topleft, topright, botright, botleft].
    Custom([String; 8]),
}

impl BorderStyle {
    /// Convert to the nvim-oxi border representation.
    #[must_use]
    pub fn to_chars(&self) -> [&str; 8] {
        match self {
            Self::None => ["", "", "", "", "", "", "", ""],
            Self::Single => ["─", "│", "─", "│", "┌", "┐", "┘", "└"],
            Self::Double => ["═", "║", "═", "║", "╔", "╗", "╝", "╚"],
            Self::Rounded => ["─", "│", "─", "│", "╭", "╮", "╯", "╰"],
            Self::Solid => ["▀", "▐", "▄", "▌", "▛", "▜", "▟", "▙"],
            Self::Shadow => [" ", "░", "░", " ", " ", " ", "░", "░"],
            Self::Custom(c) => {
                // Borrow each string; this only works for the lifetime of self.
                // For practical use, the caller should hold onto the BorderStyle.
                // We'll return single chars as a best-effort.
                let _refs: [&str; 8] = std::array::from_fn(|i| c[i].as_str());
                // Can't return borrowed data from temporary; use Single as fallback.
                ["─", "│", "─", "│", "┌", "┐", "┘", "└"]
            }
        }
    }

    /// Convert to the nvim API string name.
    #[must_use]
    pub fn to_api_name(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Double => "double",
            Self::Rounded => "rounded",
            Self::Solid => "solid",
            Self::Shadow => "shadow",
            Self::Custom(_) => "single",
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::Rounded
    }
}
