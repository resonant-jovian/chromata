use crate::{Color, Theme};
use ::ratatui::style::Color as RatColor;

impl From<Color> for RatColor {
    fn from(c: Color) -> Self {
        RatColor::Rgb(c.r, c.g, c.b)
    }
}

impl Theme {
    /// Convert to a ratatui Style for the default text.
    pub fn to_ratatui_style(&self) -> ::ratatui::style::Style {
        ::ratatui::style::Style::default()
            .fg(self.fg.into())
            .bg(self.bg.into())
    }
}
