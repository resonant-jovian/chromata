//! Framework conversion traits.
//!
//! Provides generic traits for converting chromata types into
//! framework-specific color types, gated behind optional features.

/// Trait for converting a Chromata [`Color`](crate::Color) to a framework-specific color type.
///
/// Implemented automatically when a framework integration feature is enabled
/// (e.g., `ratatui-integration`). You typically use `From`/`Into` directly.
///
/// # Examples
///
/// ```rust,ignore
/// // With ratatui-integration feature enabled:
/// use chromata::Color;
///
/// let c = Color::new(255, 0, 0);
/// let ratatui_color: ratatui::style::Color = c.into();
/// ```
pub trait IntoFrameworkColor<T> {
    /// Convert this color into the framework's color type.
    fn into_framework_color(self) -> T;
}

/// Trait for converting an entire Chromata [`Theme`](crate::Theme) into a framework theme.
///
/// # Examples
///
/// ```rust,ignore
/// // With ratatui-integration feature enabled:
/// let theme = &chromata::popular::gruvbox::DARK_HARD;
/// let style = theme.as_framework_theme();
/// ```
pub trait IntoFrameworkTheme<T> {
    /// Convert this theme into the framework's theme representation.
    fn as_framework_theme(&self) -> T;
}
