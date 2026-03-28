//! owo-colors terminal coloring integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`owo_colors::Rgb`].

use crate::Color;

/// Convert a chromata Color to an owo-colors Rgb.
impl From<Color> for ::owo_colors::Rgb {
    fn from(c: Color) -> Self {
        ::owo_colors::Rgb(c.r, c.g, c.b)
    }
}
