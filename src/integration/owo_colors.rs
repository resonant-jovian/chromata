//! owo-colors terminal coloring integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`owo_colors::Rgb`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "1", features = ["owo-colors-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: owo_colors::Rgb = theme.fg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to an owo-colors Rgb.
impl From<Color> for ::owo_colors::Rgb {
    fn from(c: Color) -> Self {
        ::owo_colors::Rgb(c.r, c.g, c.b)
    }
}
