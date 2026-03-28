//! Macroquad game framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`macroquad::color::Color`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["macroquad-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: macroquad::color::Color = theme.bg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a macroquad RGBA color (alpha = 1.0).
impl From<Color> for ::macroquad::color::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::macroquad::color::Color::new(r, g, b, 1.0)
    }
}
