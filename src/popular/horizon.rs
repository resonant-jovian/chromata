//! Horizon color themes.
//!
//! A beautifully warm dark and light color scheme by Jonathan Olaleye.
//! <https://github.com/jolaleye/horizon-theme-vscode>

use crate::{Color, Contrast, Theme, Variant};

/// Horizon Dark — dark variant.
///
/// Author: Jonathan Olaleye
/// Variant: Dark
/// Contrast: High
/// Source: horizon-theme-vscode
pub const DARK: Theme = Theme {
    name: "Horizon Dark",
    author: "Jonathan Olaleye",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x1c1e26),
    fg: Color::from_hex(0xd5d8da),
    cursor: Some(Color::from_hex(0xd5d8da)),
    selection: Some(Color::from_hex(0x2e303e)),
    line_highlight: Some(Color::from_hex(0x232530)),
    gutter: Some(Color::from_hex(0x6c6f93)),
    statusbar_bg: Some(Color::from_hex(0x232530)),
    statusbar_fg: Some(Color::from_hex(0x6c6f93)),
    comment: Some(Color::from_hex(0x6c6f93)),
    keyword: Some(Color::from_hex(0xb877db)),
    string: Some(Color::from_hex(0xfab795)),
    function: Some(Color::from_hex(0x25b0bc)),
    variable: Some(Color::from_hex(0xe95678)),
    r#type: Some(Color::from_hex(0xfac29a)),
    constant: Some(Color::from_hex(0xf09483)),
    operator: Some(Color::from_hex(0x09f7a0)),
    tag: Some(Color::from_hex(0xe95678)),
    error: Some(Color::from_hex(0xe95678)),
    warning: Some(Color::from_hex(0xfac29a)),
    info: Some(Color::from_hex(0x25b0bc)),
    success: Some(Color::from_hex(0x09f7a0)),
    red: Some(Color::from_hex(0xe95678)),
    orange: Some(Color::from_hex(0xfab795)),
    yellow: Some(Color::from_hex(0xfac29a)),
    green: Some(Color::from_hex(0x09f7a0)),
    cyan: Some(Color::from_hex(0x25b0bc)),
    blue: Some(Color::from_hex(0x26bbd9)),
    purple: Some(Color::from_hex(0xb877db)),
    magenta: Some(Color::from_hex(0xe95678)),
};

/// Horizon Light — light variant.
///
/// Author: Jonathan Olaleye
/// Variant: Light
/// Contrast: Normal
/// Source: horizon-theme-vscode
pub const LIGHT: Theme = Theme {
    name: "Horizon Light",
    author: "Jonathan Olaleye",
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfdf0ed),
    fg: Color::from_hex(0x403c3d),
    cursor: Some(Color::from_hex(0x403c3d)),
    selection: Some(Color::from_hex(0xf9e8e4)),
    line_highlight: Some(Color::from_hex(0xfaddd6)),
    gutter: Some(Color::from_hex(0xbdb3b1)),
    statusbar_bg: Some(Color::from_hex(0xfaddd6)),
    statusbar_fg: Some(Color::from_hex(0x948c8a)),
    comment: Some(Color::from_hex(0xbdb3b1)),
    keyword: Some(Color::from_hex(0xa751b8)),
    string: Some(Color::from_hex(0xe58d7d)),
    function: Some(Color::from_hex(0x1d8991)),
    variable: Some(Color::from_hex(0xda103f)),
    r#type: Some(Color::from_hex(0xf6661e)),
    constant: Some(Color::from_hex(0xe96862)),
    operator: Some(Color::from_hex(0x07da8c)),
    tag: Some(Color::from_hex(0xda103f)),
    error: Some(Color::from_hex(0xda103f)),
    warning: Some(Color::from_hex(0xf6661e)),
    info: Some(Color::from_hex(0x1d8991)),
    success: Some(Color::from_hex(0x07da8c)),
    red: Some(Color::from_hex(0xda103f)),
    orange: Some(Color::from_hex(0xf6661e)),
    yellow: Some(Color::from_hex(0xf7c577)),
    green: Some(Color::from_hex(0x07da8c)),
    cyan: Some(Color::from_hex(0x1d8991)),
    blue: Some(Color::from_hex(0x1d8991)),
    purple: Some(Color::from_hex(0xa751b8)),
    magenta: Some(Color::from_hex(0xda103f)),
};
