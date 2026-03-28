//! Comfy-table styled table integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`comfy_table::Color`] and a
//! convenience method on [`Theme`](crate::Theme).

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
    pub fn style_comfy_cell(&self, cell: ::comfy_table::Cell) -> ::comfy_table::Cell {
        cell.fg(self.fg.into()).bg(self.bg.into())
    }
}
