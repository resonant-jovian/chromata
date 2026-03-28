//! Syntect syntax highlighting integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`syntect::highlighting::Color`]
//! and a convenience method on [`Theme`](crate::Theme).

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
