//! Palette color science integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`palette::Srgb<u8>`].

use crate::Color;

/// Convert a chromata Color to a palette sRGB color with u8 components.
impl From<Color> for ::palette::Srgb<u8> {
    fn from(c: Color) -> Self {
        ::palette::Srgb::new(c.r, c.g, c.b)
    }
}
