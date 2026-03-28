//! Framework integration modules.
//!
//! Each sub-module provides [`From<Color>`](crate::Color) conversions for a
//! specific UI framework, gated behind an optional feature flag. Some modules
//! also provide convenience methods on [`Theme`](crate::Theme).
//!
//! ## Available integrations
//!
//! | Feature | Target type | Category |
//! |---------|------------|----------|
//! | `bevy-color-integration` | [`bevy_color::Srgba`] | Game engine |
//! | `colored-integration` | [`colored::Color`] | Terminal |
//! | `comfy-table-integration` | [`comfy_table::Color`] | Terminal |
//! | `crossterm-integration` | [`crossterm::style::Color`] | Terminal |
//! | `cursive-integration` | [`cursive_core::theme::Color`] | TUI |
//! | `egui-integration` | [`egui::Color32`] | GUI |
//! | `iced-integration` | [`iced_core::Color`] | GUI |
//! | `image-integration` | [`image::Rgb<u8>`] | Image |
//! | `macroquad-integration` | [`macroquad::color::Color`] | Game engine |
//! | `owo-colors-integration` | [`owo_colors::Rgb`] | Terminal |
//! | `palette-integration` | [`palette::Srgb<u8>`] | Color science |
//! | `plotters-integration` | [`plotters::style::RGBColor`] | Charting |
//! | `ratatui-integration` | [`ratatui::style::Color`] | TUI |
//! | `slint-integration` | [`slint::Color`] | GUI |
//! | `syntect-integration` | [`syntect::highlighting::Color`] | Syntax |
//! | `termion-integration` | [`termion::color::Rgb`] | Terminal |
//! | `tiny-skia-integration` | [`tiny_skia::PremultipliedColorU8`] | Rendering |
//! | `wgpu-integration` | [`wgpu::Color`] | Rendering |

#[cfg(feature = "bevy-color-integration")]
mod bevy_color;

#[cfg(feature = "colored-integration")]
mod colored;

#[cfg(feature = "comfy-table-integration")]
mod comfy_table;

#[cfg(feature = "crossterm-integration")]
mod crossterm;

#[cfg(feature = "cursive-integration")]
mod cursive;

#[cfg(feature = "egui-integration")]
mod egui;

#[cfg(feature = "iced-integration")]
mod iced;

#[cfg(feature = "image-integration")]
mod image;

#[cfg(feature = "macroquad-integration")]
mod macroquad;

#[cfg(feature = "owo-colors-integration")]
mod owo_colors;

#[cfg(feature = "palette-integration")]
mod palette;

#[cfg(feature = "plotters-integration")]
mod plotters;

#[cfg(feature = "ratatui-integration")]
mod ratatui;

#[cfg(feature = "slint-integration")]
mod slint;

#[cfg(feature = "syntect-integration")]
mod syntect;

#[cfg(feature = "termion-integration")]
mod termion;

#[cfg(feature = "tiny-skia-integration")]
mod tiny_skia;

#[cfg(feature = "wgpu-integration")]
mod wgpu;
