//! Framework integration modules.
//!
//! Each sub-module provides [`From<Color>`](crate::Color) conversions for a
//! specific UI framework, gated behind an optional feature flag.

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
