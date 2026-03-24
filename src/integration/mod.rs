//! Framework integration modules.
//!
//! Each sub-module provides [`From<Color>`](crate::Color) conversions for a
//! specific UI framework, gated behind an optional feature flag.

#[cfg(feature = "ratatui-integration")]
mod ratatui;

#[cfg(feature = "egui-integration")]
mod egui;

#[cfg(feature = "crossterm-integration")]
mod crossterm;

#[cfg(feature = "iced-integration")]
mod iced;
