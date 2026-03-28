//! Termion terminal I/O integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`termion::color::Rgb`].

use crate::Color;

/// Convert a chromata Color to a termion Rgb color.
impl From<Color> for ::termion::color::Rgb {
    fn from(c: Color) -> Self {
        ::termion::color::Rgb(c.r, c.g, c.b)
    }
}
