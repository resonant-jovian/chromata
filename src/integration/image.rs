//! Image crate integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`image::Rgb<u8>`].

use crate::Color;

/// Convert a chromata Color to an image Rgb pixel.
impl From<Color> for ::image::Rgb<u8> {
    fn from(c: Color) -> Self {
        ::image::Rgb([c.r, c.g, c.b])
    }
}
