//! # Chromata
//!
//! **1000+ editor color themes as compile-time Rust constants.**
//!
//! Chromata provides every popular editor and terminal color theme as
//! compile-time `const` data. No file parsing, no runtime allocation,
//! no dependencies. Add `chromata` to your `Cargo.toml`, write
//! `chromata::popular::gruvbox::DARK_HARD.bg`, and get a hex color
//! at zero cost.
//!
//! ## Feature Flags
//!
//! | Feature | Themes | Description |
//! |---------|--------|-------------|
//! | `popular` (default) | 49 | Curated best themes (gruvbox, catppuccin, nord...) |
//! | `base16` | 305 | Base16 themes from tinted-theming/schemes |
//! | `base24` | 184 | Base24 themes from tinted-theming/schemes |
//! | `vim` | 464 | Vim colorschemes from vim-colorschemes repos |
//! | `emacs` | 102 | Emacs themes from emacs-themes-site |
//! | `all` | 1104 | All collections combined |
//! | `ratatui-integration` | — | `From<Color>` for ratatui types |
//! | `egui-integration` | — | `From<Color>` for egui types |
//! | `crossterm-integration` | — | `From<Color>` for crossterm types |
//! | `iced-integration` | — | `From<Color>` for iced types |
//! | `serde-support` | — | Serialize/deserialize themes |
//!
//! ## Quick Start
//!
//! ```rust
//! use chromata::popular::gruvbox;
//!
//! let theme = gruvbox::DARK_HARD;
//! println!("Background: {}", theme.bg.to_css_hex());
//! println!("Is dark? {}", theme.is_dark());
//! ```
//!
//! ## Query APIs
//!
//! ```rust
//! use chromata::{find_by_name, filter_by_variant, Variant};
//!
//! if let Some(theme) = find_by_name("Gruvbox Dark Hard") {
//!     println!("{}: {}", theme.name, theme.bg);
//! }
//! let dark_themes = filter_by_variant(Variant::Dark);
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![allow(rustdoc::bare_urls)]
extern crate alloc;

mod iter;
mod traits;
mod types;

pub mod prelude;

pub use iter::*;
pub use traits::*;
pub use types::*;

#[cfg(feature = "base16")]
pub mod base16;

#[cfg(feature = "base24")]
pub mod base24;

#[cfg(feature = "vim")]
pub mod vim;

#[cfg(feature = "emacs")]
pub mod emacs;

#[cfg(feature = "popular")]
pub mod popular;

#[cfg(any(
    feature = "ratatui-integration",
    feature = "egui-integration",
    feature = "crossterm-integration",
    feature = "iced-integration",
))]
mod integration;
