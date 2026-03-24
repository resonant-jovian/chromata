use crate::Theme;

/// All themes enabled by current feature flags, as a static slice.
pub fn all_themes() -> &'static [&'static Theme] {
    ALL_THEMES
}

// Build the static theme slice from all enabled features.
// Each feature module exposes a THEMES constant that lists its themes.
static ALL_THEMES: &[&Theme] = {
    // Use a nested const block to build the combined slice.
    // For now, we concatenate at runtime via the function below.
    // This will be replaced by a macro or build.rs generation
    // once theme counts grow large.
    &[]
};

/// Collect all themes from enabled features into a Vec.
///
/// Unlike `all_themes()` which returns a static slice, this allocates
/// and can dynamically combine themes from all enabled feature modules.
pub fn collect_all_themes() -> alloc::vec::Vec<&'static Theme> {
    let mut themes = alloc::vec::Vec::new();

    #[cfg(feature = "popular")]
    themes.extend_from_slice(crate::popular::THEMES);

    #[cfg(feature = "base16")]
    themes.extend_from_slice(crate::base16::THEMES);

    #[cfg(feature = "base24")]
    themes.extend_from_slice(crate::base24::THEMES);

    #[cfg(feature = "vim")]
    themes.extend_from_slice(crate::vim::THEMES);

    #[cfg(feature = "emacs")]
    themes.extend_from_slice(crate::emacs::THEMES);

    themes
}
