//! Ratatui framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`ratatui::style::Color`] and a
//! convenience method on [`Theme`](crate::Theme).
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["ratatui-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let bg: ratatui::style::Color = theme.bg.into();
//! let fg: ratatui::style::Color = theme.fg.into();
//! ```
//!
//! # Convenience
//!
//! ```rust,ignore
//! let style = theme.to_ratatui_style();
//! // style has fg and bg set from the theme
//! ```

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
