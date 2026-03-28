//! Colored terminal text integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`colored::Color`] and a
//! convenience method on [`Theme`](crate::Theme).
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["colored-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: colored::Color = theme.fg.into();
//! ```
//!
//! # Convenience
//!
//! ```rust,ignore
//! let styled = theme.colorize("hello world");
//! // ColoredString with theme fg and bg applied
//! ```

use crate::{Color, Theme};

/// Convert a chromata Color to a colored TrueColor.
impl From<Color> for ::colored::Color {
    fn from(c: Color) -> Self {
        ::colored::Color::TrueColor {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}

impl Theme {
    /// Apply this theme's foreground and background to a string.
    pub fn colorize(&self, text: &str) -> ::colored::ColoredString {
        use ::colored::Colorize;
        text.truecolor(self.fg.r, self.fg.g, self.fg.b)
            .on_truecolor(self.bg.r, self.bg.g, self.bg.b)
    }
}
