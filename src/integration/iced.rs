use crate::Color;

impl From<Color> for ::iced_core::Color {
    fn from(c: Color) -> Self {
        let (r, g, b) = c.to_f32();
        ::iced_core::Color::new(r, g, b, 1.0)
    }
}
