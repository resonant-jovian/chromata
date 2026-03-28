//! Plotters charting library integration.
//!
//! Provides [`From<Color>`](crate::Color) for [`plotters::style::RGBColor`] and a
//! convenience method on [`Theme`](crate::Theme).

use alloc::vec::Vec;

use crate::{Color, Theme};

/// Convert a chromata Color to a plotters RGBColor.
impl From<Color> for ::plotters::style::RGBColor {
    fn from(c: Color) -> Self {
        ::plotters::style::RGBColor(c.r, c.g, c.b)
    }
}

impl Theme {
    /// Return accent colors as plotters RGBColors for chart series.
    ///
    /// Collects defined accent colors (red, orange, yellow, green, cyan,
    /// blue, purple, magenta) in palette order, skipping any that are `None`.
    pub fn plotters_series_colors(&self) -> Vec<::plotters::style::RGBColor> {
        let accents = [
            self.red,
            self.orange,
            self.yellow,
            self.green,
            self.cyan,
            self.blue,
            self.purple,
            self.magenta,
        ];
        accents
            .iter()
            .filter_map(|c| c.map(::plotters::style::RGBColor::from))
            .collect()
    }
}
