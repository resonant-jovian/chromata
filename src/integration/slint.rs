//! Slint UI framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`slint::Color`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "1", features = ["slint-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: slint::Color = theme.bg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a Slint RGB color.
impl From<Color> for ::slint::Color {
    fn from(c: Color) -> Self {
        ::slint::Color::from_rgb_u8(c.r, c.g, c.b)
    }
}
