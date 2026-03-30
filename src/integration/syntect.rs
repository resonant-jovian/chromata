//! Syntect syntax highlighting integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`syntect::highlighting::Color`]
//! and a convenience method on [`Theme`](crate::Theme).
//!
//! # Enable
//!
//! ```toml
//! [dependencies]
//! chromata = { version = "1", features = ["syntect-integration"] }
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! let color: syntect::highlighting::Color = theme.fg.into();
//! ```
//!
//! # Convenience
//!
//! ```rust,ignore
//! let settings = theme.to_syntect_theme_settings();
//! // settings.foreground, settings.background, settings.caret, etc.
//! ```

use crate::{Color, Theme};

/// Convert a chromata Color to a syntect RGBA color (alpha = 255).
impl From<Color> for ::syntect::highlighting::Color {
    fn from(c: Color) -> Self {
        ::syntect::highlighting::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: 255,
        }
    }
}

impl Theme {
    /// Convert to syntect ThemeSettings with mapped UI colors.
    pub fn to_syntect_theme_settings(&self) -> ::syntect::highlighting::ThemeSettings {
        ::syntect::highlighting::ThemeSettings {
            foreground: Some(self.fg.into()),
            background: Some(self.bg.into()),
            caret: self.cursor.map(Into::into),
            line_highlight: self.line_highlight.map(Into::into),
            selection: self.selection.map(Into::into),
            gutter: self.gutter.map(Into::into),
            ..Default::default()
        }
    }
}
