//! # Chromata
//!
//! **1000+ editor color themes as compile-time Rust constants.**
//!
//! Chromata provides every popular editor and terminal color theme as
//! compile-time `const` data. No file parsing, no runtime allocation,
//! one dependency (`libm` for `no_std` math). Add `chromata` to your `Cargo.toml`, write
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
//! | `bevy-color-integration` | — | `From<Color>` for bevy_color types |
//! | `colored-integration` | — | `From<Color>` for colored types |
//! | `comfy-table-integration` | — | `From<Color>` for comfy-table types |
//! | `crossterm-integration` | — | `From<Color>` for crossterm types |
//! | `cursive-integration` | — | `From<Color>` for cursive types |
//! | `egui-integration` | — | `From<Color>` for egui types |
//! | `iced-integration` | — | `From<Color>` for iced types |
//! | `image-integration` | — | `From<Color>` for image types |
//! | `macroquad-integration` | — | `From<Color>` for macroquad types |
//! | `owo-colors-integration` | — | `From<Color>` for owo-colors types |
//! | `palette-integration` | — | `From<Color>` for palette types |
//! | `plotters-integration` | — | `From<Color>` for plotters types |
//! | `ratatui-integration` | — | `From<Color>` for ratatui types |
//! | `slint-integration` | — | `From<Color>` for slint types |
//! | `syntect-integration` | — | `From<Color>` for syntect types |
//! | `termion-integration` | — | `From<Color>` for termion types |
//! | `tiny-skia-integration` | — | `From<Color>` for tiny-skia types |
//! | `wgpu-integration` | — | `From<Color>` for wgpu types |
//! | `serde-support` | — | Serialize/deserialize themes |
//!
//! ## Core Types
//!
//! - [`Color`] — RGB color with hex conversion, WCAG luminance, contrast ratio, lerp
//! - [`Theme`] — 29 color fields + metadata (name, author, variant, contrast)
//! - [`Variant`] — `Dark` or `Light`
//! - [`Contrast`] — `High`, `Normal`, or `Low` (WCAG-based)
//! - [`Base16Palette`] — The 16 base16 palette slots (available on base16 themes)
//! - [`Base24Palette`] — Extended 24-slot palette (available on base24 themes)
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
// Generated theme files contain bare URLs in author fields.
#![allow(rustdoc::bare_urls)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
extern crate alloc;

mod iter;
mod types;

pub mod prelude;

pub use iter::*;
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
    feature = "bevy-color-integration",
    feature = "colored-integration",
    feature = "comfy-table-integration",
    feature = "crossterm-integration",
    feature = "cursive-integration",
    feature = "egui-integration",
    feature = "iced-integration",
    feature = "image-integration",
    feature = "macroquad-integration",
    feature = "owo-colors-integration",
    feature = "palette-integration",
    feature = "plotters-integration",
    feature = "ratatui-integration",
    feature = "slint-integration",
    feature = "syntect-integration",
    feature = "termion-integration",
    feature = "tiny-skia-integration",
    feature = "wgpu-integration",
))]
mod integration;
