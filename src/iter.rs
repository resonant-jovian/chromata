use crate::Theme;
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
