//! Vim color themes (~600 schemes).
//!
//! Themes parsed from `.vim` colorscheme files sourced from
//! [flazz/vim-colorschemes](https://github.com/flazz/vim-colorschemes) and
//! the official [vim/colorschemes](https://github.com/vim/colorschemes) repository.
//!
//! Enable with the `vim` feature flag. Populate by running
//! `cargo xtask generate`.

use crate::Theme;

/// All themes in the `vim` collection.
pub static THEMES: &[&Theme] = &[];
