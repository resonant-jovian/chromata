//! Crossterm framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`crossterm::style::Color`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["crossterm-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let bg: crossterm::style::Color = theme.bg.into();
//! let fg: crossterm::style::Color = theme.fg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a crossterm RGB color.
impl From<Color> for ::crossterm::style::Color {
    fn from(c: Color) -> Self {
        ::crossterm::style::Color::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}
