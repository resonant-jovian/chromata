//! Curated collection of the most popular editor themes.
//!
//! This module contains a hand-picked selection of ~50 widely-used themes.
//! It is enabled by default via the `popular` feature flag.

pub mod gruvbox;

use crate::Theme;

/// All themes in the `popular` collection.
pub static THEMES: &[&Theme] = &[
    &gruvbox::DARK_HARD,
    &gruvbox::DARK,
    &gruvbox::LIGHT,
];
