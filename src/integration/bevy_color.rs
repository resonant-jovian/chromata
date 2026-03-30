//! Bevy game engine color integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`bevy_color::Srgba`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "1", features = ["bevy-color-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: bevy_color::Srgba = theme.bg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a Bevy sRGBA color (alpha = 1.0).
impl From<Color> for ::bevy_color::Srgba {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::bevy_color::Srgba::new(r, g, b, 1.0)
    }
}
