//! Iced framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`iced_core::Color`].

use crate::Color;

/// Convert a chromata Color to an iced RGBA color (alpha = 1.0).
impl From<Color> for ::iced_core::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::iced_core::Color { r, g, b, a: 1.0 }
    }
}
