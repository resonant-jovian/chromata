//! wgpu graphics API integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`wgpu::Color`].

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
