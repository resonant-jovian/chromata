use crate::Color;

impl From<Color> for ::crossterm::style::Color {
    fn from(c: Color) -> Self {
        ::crossterm::style::Color::Rgb {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }
}
