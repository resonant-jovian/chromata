//! Image crate integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`image::Rgb<u8>`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "0.3.0", features = ["image-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let pixel: image::Rgb<u8> = theme.bg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to an image Rgb pixel.
impl From<Color> for ::image::Rgb<u8> {
    fn from(c: Color) -> Self {
        ::image::Rgb([c.r, c.g, c.b])
    }
}
