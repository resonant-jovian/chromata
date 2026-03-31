//! Comfy-table styled table integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`comfy_table::Color`] and a
//! convenience method on [`Theme`](crate::Theme).
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "1", features = ["comfy-table-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: comfy_table::Color = theme.bg.into();
//! ```
//!
//! # Convenience
//!
//! ```rust,ignore
//! use comfy_table::Cell;
//! let cell = theme.to_comfy_table_cell(Cell::new("text"));
//! // Cell with theme fg and bg applied
//! ```

use crate::{Color, Theme};

/// Convert a chromata Color to a comfy-table RGB color.
impl From<Color> for ::comfy_table::Color {
    fn from(c: Color) -> Self {
        ::comfy_table::Color::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

impl Theme {
    /// Apply this theme's foreground and background to a table cell.
    #[must_use]
    pub fn to_comfy_table_cell(&self, cell: ::comfy_table::Cell) -> ::comfy_table::Cell {
        cell.fg(self.fg.into()).bg(self.bg.into())
    }
}
