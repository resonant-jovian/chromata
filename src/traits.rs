/// Trait for converting a Chromata [`Color`](crate::Color) to a framework-specific color type.
pub trait IntoFrameworkColor<T> {
    /// Convert this color into the framework's color type.
    fn into_framework_color(self) -> T;
}

/// Trait for converting an entire Chromata [`Theme`](crate::Theme) into a framework theme.
pub trait IntoFrameworkTheme<T> {
    /// Convert this theme into the framework's theme representation.
    fn as_framework_theme(&self) -> T;
}
