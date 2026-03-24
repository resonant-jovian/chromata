//! Curated collection of the most popular editor themes.
//!
//! This module contains a hand-picked selection of ~50 widely-used themes.
//! It is enabled by default via the `popular` feature flag.

pub mod ayu;
pub mod catppuccin;
pub mod dracula;
pub mod everforest;
pub mod flexoki;
pub mod github;
pub mod gruvbox;
pub mod horizon;
pub mod kanagawa;
pub mod material;
pub mod monokai;
pub mod moonlight;
pub mod nightfox;
pub mod nord;
pub mod one_dark;
pub mod poimandres;
pub mod rose_pine;
pub mod snazzy;
pub mod solarized;
pub mod synthwave;
pub mod tokyo_night;
pub mod tomorrow;
pub mod vesper;
pub mod zenburn;

use crate::Theme;

/// All themes in the `popular` collection.
pub static THEMES: &[&Theme] = &[
    &ayu::DARK,
    &ayu::MIRAGE,
    &ayu::LIGHT,
    &catppuccin::LATTE,
    &catppuccin::FRAPPE,
    &catppuccin::MACCHIATO,
    &catppuccin::MOCHA,
    &dracula::THEME,
    &everforest::DARK,
    &everforest::LIGHT,
    &flexoki::DARK,
    &flexoki::LIGHT,
    &github::DARK,
    &github::LIGHT,
    &gruvbox::DARK_HARD,
    &gruvbox::DARK,
    &gruvbox::LIGHT,
    &horizon::DARK,
    &horizon::LIGHT,
    &kanagawa::WAVE,
    &kanagawa::DRAGON,
    &material::THEME,
    &material::DARKER,
    &material::PALENIGHT,
    &material::LIGHTER,
    &monokai::THEME,
    &monokai::PRO,
    &moonlight::THEME,
    &nightfox::NIGHTFOX,
    &nightfox::DAWNFOX,
    &nord::THEME,
    &one_dark::THEME,
    &one_dark::LIGHT,
    &poimandres::THEME,
    &rose_pine::THEME,
    &rose_pine::MOON,
    &rose_pine::DAWN,
    &snazzy::THEME,
    &solarized::DARK,
    &solarized::LIGHT,
    &synthwave::THEME,
    &tokyo_night::DARK,
    &tokyo_night::STORM,
    &tokyo_night::LIGHT,
    &tomorrow::NIGHT,
    &tomorrow::NIGHT_EIGHTIES,
    &tomorrow::LIGHT,
    &vesper::THEME,
    &zenburn::THEME,
];
