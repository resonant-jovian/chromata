use crate::{Contrast, Theme, Variant};
use alloc::vec::Vec;

/// Collect all themes from enabled feature modules into a Vec.
///
/// Aggregates themes from every enabled feature flag (`popular`, `base16`,
/// `base24`, `vim`, `emacs`). For zero-allocation access to a specific
/// collection, use the module-level `THEMES` slices directly
/// (e.g., `chromata::popular::THEMES`).
#[allow(unused_mut)]
pub fn collect_all_themes() -> Vec<&'static Theme> {
    let mut themes = Vec::new();

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

/// Find a theme by exact name (case-sensitive).
///
/// Searches all enabled feature modules. Returns the first match.
///
/// # Examples
/// ```
/// let theme = chromata::find_by_name("Gruvbox Dark Hard");
/// assert!(theme.is_some());
/// ```
pub fn find_by_name(name: &str) -> Option<&'static Theme> {
    collect_all_themes().into_iter().find(|t| t.name == name)
}

/// Filter all themes by variant (Dark or Light).
///
/// Returns themes matching the given variant across all enabled
/// feature modules.
pub fn filter_by_variant(variant: Variant) -> Vec<&'static Theme> {
    collect_all_themes()
        .into_iter()
        .filter(|t| t.variant == variant)
        .collect()
}

/// Filter all themes by contrast level.
///
/// Returns themes matching the given contrast classification across
/// all enabled feature modules.
pub fn filter_by_contrast(contrast: Contrast) -> Vec<&'static Theme> {
    collect_all_themes()
        .into_iter()
        .filter(|t| t.contrast == contrast)
        .collect()
}
