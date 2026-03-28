//! Bevy game engine color integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`bevy_color::Srgba`].

use crate::Color;

/// Convert a chromata Color to a Bevy sRGBA color (alpha = 1.0).
impl From<Color> for ::bevy_color::Srgba {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::bevy_color::Srgba::new(r, g, b, 1.0)
    }
}
