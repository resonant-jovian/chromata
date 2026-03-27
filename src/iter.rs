use crate::{Contrast, Theme, Variant};
use alloc::vec::Vec;

/// Iterate all themes from enabled feature modules without allocation.
///
/// Calls `f` once for each theme across all enabled collections.
/// Useful when you want to search or aggregate without building a Vec.
#[allow(unused_variables)]
fn for_each_theme(mut f: impl FnMut(&'static Theme)) {
    #[cfg(feature = "popular")]
    for theme in crate::popular::THEMES {
        f(theme);
    }

    #[cfg(feature = "base16")]
    for theme in crate::base16::THEMES {
        f(theme);
    }

    #[cfg(feature = "base24")]
    for theme in crate::base24::THEMES {
        f(theme);
    }

    #[cfg(feature = "vim")]
    for theme in crate::vim::THEMES {
        f(theme);
    }

    #[cfg(feature = "emacs")]
    for theme in crate::emacs::THEMES {
        f(theme);
    }
}

/// Collect all themes from enabled feature modules into a Vec.
///
/// Aggregates themes from every enabled feature flag (`popular`, `base16`,
/// `base24`, `vim`, `emacs`). For zero-allocation access to a specific
/// collection, use the module-level `THEMES` slices directly
/// (e.g., `chromata::popular::THEMES`).
///
/// # Examples
///
/// ```
/// let all = chromata::collect_all_themes();
/// assert!(!all.is_empty());
/// ```
pub fn collect_all_themes() -> Vec<&'static Theme> {
    let mut themes = Vec::new();
    for_each_theme(|t| themes.push(t));
    themes
}

/// Find a theme by exact name (case-sensitive).
///
/// Searches all enabled feature modules without allocation. Returns the
/// first match.
///
/// # Examples
/// ```
/// let theme = chromata::find_by_name("Gruvbox Dark Hard");
/// assert!(theme.is_some());
/// ```
pub fn find_by_name(name: &str) -> Option<&'static Theme> {
    let mut result = None;
    for_each_theme(|t| {
        if result.is_none() && t.name == name {
            result = Some(t);
        }
    });
    result
}

/// Filter all themes by variant (Dark or Light).
///
/// Returns themes matching the given variant across all enabled
/// feature modules.
///
/// # Examples
///
/// ```
/// use chromata::Variant;
///
/// let dark = chromata::filter_by_variant(Variant::Dark);
/// assert!(dark.iter().all(|t| t.variant == Variant::Dark));
/// ```
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
///
/// # Examples
///
/// ```
/// use chromata::Contrast;
///
/// let high = chromata::filter_by_contrast(Contrast::High);
/// assert!(high.iter().all(|t| t.contrast == Contrast::High));
/// ```
pub fn filter_by_contrast(contrast: Contrast) -> Vec<&'static Theme> {
    collect_all_themes()
        .into_iter()
        .filter(|t| t.contrast == contrast)
        .collect()
}
