//! # Chromata
//!
//! **1000+ editor color themes as compile-time Rust constants.**
//!
//! Chromata provides every popular editor and terminal color theme as
//! compile-time `const` data. No file parsing, no runtime allocation,
//! no dependencies. Add `chromata` to your `Cargo.toml`, write
//! `chromata::popular::gruvbox::DARK_HARD.bg()`, and get a hex color
//! at zero cost.
//!
//! ## Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `popular` (default) | Curated ~50 best themes |
//! | `base16` | ~305 base16 themes |
//! | `base24` | ~184 base24 themes |
//! | `vim` | ~600 vim themes |
//! | `emacs` | ~200 emacs themes |
//! | `all` | Everything |
//! | `ratatui-integration` | `From<Color>` for ratatui types |
//! | `egui-integration` | `From<Color>` for egui types |
//! | `crossterm-integration` | `From<Color>` for crossterm types |
//! | `iced-integration` | `From<Color>` for iced types |
//! | `serde-support` | Serialize/deserialize themes |
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

#![no_std]
extern crate alloc;

mod types;
mod traits;
mod iter;

pub use types::*;
pub use traits::*;
pub use iter::*;

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
