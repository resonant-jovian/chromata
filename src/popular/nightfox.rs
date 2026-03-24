//! Nightfox color themes.
//!
//! A highly customizable theme by EdenEast.
//! <https://github.com/EdenEast/nightfox.nvim>

use crate::{Color, Contrast, Theme, Variant};

/// Nightfox — dark variant.
///
/// Author: EdenEast
/// Variant: Dark
/// Contrast: High
/// Source: nightfox.nvim
pub const NIGHTFOX: Theme = Theme {
    name: "Nightfox",
    author: "EdenEast",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x192330),
    fg: Color::from_hex(0xcdcecf),
    cursor: Some(Color::from_hex(0xcdcecf)),
    selection: Some(Color::from_hex(0x2b3b51)),
    line_highlight: Some(Color::from_hex(0x212e3f)),
    gutter: Some(Color::from_hex(0x738091)),
    statusbar_bg: Some(Color::from_hex(0x131a24)),
    statusbar_fg: Some(Color::from_hex(0xaeafb0)),
    comment: Some(Color::from_hex(0x738091)),
    keyword: Some(Color::from_hex(0x9d79d6)),
    string: Some(Color::from_hex(0x81b29a)),
    function: Some(Color::from_hex(0x719cd6)),
    variable: Some(Color::from_hex(0xdbc074)),
    r#type: Some(Color::from_hex(0x63cdcf)),
    constant: Some(Color::from_hex(0xf4a261)),
    operator: Some(Color::from_hex(0xdbc074)),
    tag: Some(Color::from_hex(0xc94f6d)),
    error: Some(Color::from_hex(0xc94f6d)),
    warning: Some(Color::from_hex(0xdbc074)),
    info: Some(Color::from_hex(0x719cd6)),
    success: Some(Color::from_hex(0x81b29a)),
    red: Some(Color::from_hex(0xc94f6d)),
    orange: Some(Color::from_hex(0xf4a261)),
    yellow: Some(Color::from_hex(0xdbc074)),
    green: Some(Color::from_hex(0x81b29a)),
    cyan: Some(Color::from_hex(0x63cdcf)),
    blue: Some(Color::from_hex(0x719cd6)),
    purple: Some(Color::from_hex(0x9d79d6)),
    magenta: Some(Color::from_hex(0xc94f6d)),
};

/// Dawnfox — light variant.
///
/// Author: EdenEast
/// Variant: Light
/// Contrast: Normal
/// Source: nightfox.nvim
pub const DAWNFOX: Theme = Theme {
    name: "Dawnfox",
    author: "EdenEast",
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfaf4ed),
    fg: Color::from_hex(0x575279),
    cursor: Some(Color::from_hex(0x575279)),
    selection: Some(Color::from_hex(0xeee8d5)),
    line_highlight: Some(Color::from_hex(0xf2e9e1)),
    gutter: Some(Color::from_hex(0xa8a3b3)),
    statusbar_bg: Some(Color::from_hex(0xf2e9e1)),
    statusbar_fg: Some(Color::from_hex(0x797593)),
    comment: Some(Color::from_hex(0xa8a3b3)),
    keyword: Some(Color::from_hex(0x907aa9)),
    string: Some(Color::from_hex(0xd7827e)),
    function: Some(Color::from_hex(0x286983)),
    variable: Some(Color::from_hex(0xea9d34)),
    r#type: Some(Color::from_hex(0x56949f)),
    constant: Some(Color::from_hex(0xb4637a)),
    operator: Some(Color::from_hex(0xea9d34)),
    tag: Some(Color::from_hex(0xb4637a)),
    error: Some(Color::from_hex(0xb4637a)),
    warning: Some(Color::from_hex(0xea9d34)),
    info: Some(Color::from_hex(0x286983)),
    success: Some(Color::from_hex(0xd7827e)),
    red: Some(Color::from_hex(0xb4637a)),
    orange: Some(Color::from_hex(0xea9d34)),
    yellow: Some(Color::from_hex(0xea9d34)),
    green: Some(Color::from_hex(0xd7827e)),
    cyan: Some(Color::from_hex(0x56949f)),
    blue: Some(Color::from_hex(0x286983)),
    purple: Some(Color::from_hex(0x907aa9)),
    magenta: Some(Color::from_hex(0xb4637a)),
};
