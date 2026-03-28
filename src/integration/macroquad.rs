//! Macroquad game framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`macroquad::color::Color`].

use crate::Color;

/// Convert a chromata Color to a macroquad RGBA color (alpha = 1.0).
impl From<Color> for ::macroquad::color::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::macroquad::color::Color::new(r, g, b, 1.0)
    }
}
