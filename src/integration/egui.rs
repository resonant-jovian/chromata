//! Egui framework integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`egui::Color32`] and a
//! convenience method on [`Theme`](crate::Theme).

use crate::{Color, Theme};

/// Convert a chromata Color to an egui Color32.
impl From<Color> for ::egui::Color32 {
    fn from(c: Color) -> Self {
        ::egui::Color32::from_rgb(c.r, c.g, c.b)
    }
}

impl Theme {
    /// Apply this theme's colors to egui Visuals.
    pub fn apply_to_visuals(&self, visuals: &mut ::egui::Visuals) {
        visuals.dark_mode = self.is_dark();
        visuals.widgets.noninteractive.bg_fill = self.bg.into();
        visuals.widgets.noninteractive.fg_stroke.color = self.fg.into();
        if let Some(sel) = self.selection {
            visuals.selection.bg_fill = sel.into();
        }
    }
}
