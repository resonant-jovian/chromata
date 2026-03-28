//! Termion terminal I/O integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`termion::color::Rgb`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["termion-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: termion::color::Rgb = theme.fg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a termion Rgb color.
impl From<Color> for ::termion::color::Rgb {
    fn from(c: Color) -> Self {
        ::termion::color::Rgb(c.r, c.g, c.b)
    }
}
