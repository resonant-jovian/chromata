//! wgpu graphics API integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`wgpu::Color`].
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "1", features = ["wgpu-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: wgpu::Color = theme.bg.into();
//! ```

use crate::Color;

/// Convert a chromata Color to a wgpu RGBA color (alpha = 1.0).
impl From<Color> for ::wgpu::Color {
    fn from(c: Color) -> Self {
        ::wgpu::Color {
            r: c.r as f64 / 255.0,
            g: c.g as f64 / 255.0,
            b: c.b as f64 / 255.0,
            a: 1.0,
        }
    }
}
