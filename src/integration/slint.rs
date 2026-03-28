//! Slint UI framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`slint::Color`].

use crate::Color;

/// Convert a chromata Color to a Slint RGB color.
impl From<Color> for ::slint::Color {
    fn from(c: Color) -> Self {
        ::slint::Color::from_rgb_u8(c.r, c.g, c.b)
    }
}
