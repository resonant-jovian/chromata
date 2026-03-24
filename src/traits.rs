/// Trait for converting a Chromata Color to a framework-specific color type.
pub trait IntoFrameworkColor<T> {
    fn into_framework_color(self) -> T;
}

/// Trait for converting an entire Chromata Theme into a framework theme.
pub trait IntoFrameworkTheme<T> {
    fn as_framework_theme(&self) -> T;
}
