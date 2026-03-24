//! Emacs color themes (~200 schemes).
//!
//! Themes parsed from `.el` deftheme files sourced from
//! [srdja/emacs-themes-site](https://github.com/srdja/emacs-themes-site).
//!
//! Enable with the `emacs` feature flag. Populate by running
//! `cargo xtask generate`.

use crate::Theme;

/// All themes in the `emacs` collection.
pub static THEMES: &[&Theme] = &[];
