//! Ratatui framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`ratatui::style::Color`] and a
//! convenience method on [`Theme`](crate::Theme).

use crate::{Color, Theme};
use ::ratatui::style::Color as RatColor;

/// Convert a chromata Color to a ratatui RGB color.
impl From<Color> for RatColor {
    fn from(c: Color) -> Self {
        RatColor::Rgb(c.r, c.g, c.b)
    }
}

impl Theme {
    /// Convert to a ratatui Style for the default text.
    pub fn to_ratatui_style(&self) -> ::ratatui::style::Style {
        ::ratatui::style::Style::default()
            .fg(self.fg.into())
            .bg(self.bg.into())
    }
}
