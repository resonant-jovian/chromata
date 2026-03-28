//! tiny-skia 2D rendering integration.
//!
//! Provides [`From<Color>`](crate::Color) for
//! [`tiny_skia::PremultipliedColorU8`].

use crate::Color;

/// Convert a chromata Color to a tiny-skia premultiplied RGBA color (alpha = 255).
impl From<Color> for ::tiny_skia::PremultipliedColorU8 {
    fn from(c: Color) -> Self {
        ::tiny_skia::PremultipliedColorU8::from_rgba(c.r, c.g, c.b, 255)
            .expect("alpha 255 premultiply is always valid")
    }
}
