//! Floating window creation and management.

use crate::border::BorderStyle;
use crate::layout::FloatLayout;
use nvim_oxi::api;
use nvim_oxi::api::types::{
    WindowBorder, WindowConfig, WindowRelativeTo, WindowStyle,
};

/// A managed floating window.
pub struct FloatWindow {
    buf: nvim_oxi::api::Buffer,
    win: Option<nvim_oxi::api::Window>,
    layout: FloatLayout,
    border: BorderStyle,
    title: Option<String>,
    focusable: bool,
}

impl FloatWindow {
    /// Create a new floating window builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf: api::create_buf(false, true).expect("failed to create scratch buffer"),
            win: None,
            layout: FloatLayout::default(),
            border: BorderStyle::default(),
            title: None,
            focusable: true,
        }
    }

    /// Set the layout.
    #[must_use]
    pub fn layout(mut self, layout: FloatLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set the border style.
    #[must_use]
    pub fn border(mut self, border: BorderStyle) -> Self {
        self.border = border;
        self
    }

    /// Set a title for the window.
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set whether the window is focusable.
    #[must_use]
    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    /// Open the floating window. Returns a reference to self for chaining.
    pub fn open(&mut self) -> tane::Result<&mut Self> {
        let ui = api::list_uis().into_iter().next();
        #[allow(clippy::cast_possible_truncation)]
        let (editor_width, editor_height) = match ui {
            Some(ref u) => (u.width as u32, u.height as u32),
            None => (80, 24),
        };

        let (row, col, width, height) = self.layout.resolve(editor_width, editor_height);

        let win_border = match &self.border {
            BorderStyle::None => WindowBorder::None,
            BorderStyle::Single => WindowBorder::Single,
            BorderStyle::Double => WindowBorder::Double,
            BorderStyle::Rounded => WindowBorder::Rounded,
            BorderStyle::Solid => WindowBorder::Solid,
            BorderStyle::Shadow => WindowBorder::Shadow,
            BorderStyle::Custom(_) => WindowBorder::Single,
        };

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .row(f64::from(row))
            .col(f64::from(col))
            .width(width)
            .height(height)
            .border(win_border)
            .focusable(self.focusable)
            .style(WindowStyle::Minimal)
            .build();

        let win = api::open_win(&self.buf, true, &config)?;
        self.win = Some(win);
        Ok(self)
    }

    /// Close the floating window if it's open.
    pub fn close(&mut self) -> tane::Result<()> {
        if let Some(win) = self.win.take() {
            win.close(true)?;
        }
        Ok(())
    }

    /// Set the buffer content (replaces all lines).
    pub fn set_lines(&mut self, lines: &[&str]) -> tane::Result<()> {
        self.buf
            .set_lines(0..lines.len(), true, lines.iter().copied())?;
        Ok(())
    }

    /// Get the underlying buffer.
    #[must_use]
    pub fn buffer(&self) -> &nvim_oxi::api::Buffer {
        &self.buf
    }

    /// Get the window handle, if open.
    #[must_use]
    pub fn window(&self) -> Option<&nvim_oxi::api::Window> {
        self.win.as_ref()
    }

    /// Check if the window is currently open and valid.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.win
            .as_ref()
            .map_or(false, |w| w.is_valid())
    }
}

impl Default for FloatWindow {
    fn default() -> Self {
        Self::new()
    }
}
