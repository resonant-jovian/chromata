//! Cursive TUI framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`cursive_core::theme::Color`] and a
//! convenience method on [`Theme`](crate::Theme).

use crate::{Color, Theme};

/// Convert a chromata Color to a cursive RGB color.
impl From<Color> for ::cursive_core::theme::Color {
    fn from(c: Color) -> Self {
        ::cursive_core::theme::Color::Rgb(c.r, c.g, c.b)
    }
}

impl Theme {
    /// Apply this theme's colors to a cursive palette.
    ///
    /// Sets Background from `bg`, View from `bg`, Primary from `fg`,
    /// Highlight from `selection` (if present), and TitlePrimary from `accent()`.
    pub fn apply_to_cursive_palette(&self, palette: &mut ::cursive_core::theme::Palette) {
        use ::cursive_core::theme::PaletteColor;

        palette[PaletteColor::Background] = self.bg.into();
        palette[PaletteColor::View] = self.bg.into();
        palette[PaletteColor::Primary] = self.fg.into();
        if let Some(sel) = self.selection {
            palette[PaletteColor::Highlight] = sel.into();
        }
        palette[PaletteColor::TitlePrimary] = self.accent().into();
    }
}
