//! Iced framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`iced_core::Color`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["iced-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let bg: iced_core::Color = theme.bg.into();
//! let fg: iced_core::Color = theme.fg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to an iced RGBA color (alpha = 1.0).
impl From<Color> for ::iced_core::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::iced_core::Color { r, g, b, a: 1.0 }
    }
}
