//! Crossterm framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`crossterm::style::Color`].

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
