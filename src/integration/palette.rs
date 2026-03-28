//! Palette color science integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`palette::Srgb<u8>`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["palette-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let srgb: palette::Srgb<u8> = theme.bg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a palette sRGB color with u8 components.
impl From<Color> for ::palette::Srgb<u8> {
    fn from(c: Color) -> Self {
        ::palette::Srgb::new(c.r, c.g, c.b)
    }
}
