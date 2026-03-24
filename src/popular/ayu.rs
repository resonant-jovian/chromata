//! Ayu color themes.
//!
//! A simple theme with bright colors by Ike Ku.
//! <https://github.com/ayu-theme/ayu-colors>

use crate::{Color, Contrast, Theme, Variant};

/// Ayu Dark — the dark variant.
///
/// Author: Ike Ku
/// Variant: Dark
/// Contrast: Normal
/// Source: ayu-colors
pub const DARK: Theme = Theme {
    name: "Ayu Dark",
    author: "Ike Ku",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x0a0e14),
    fg: Color::from_hex(0xb3b1ad),
    cursor: Some(Color::from_hex(0xe6b450)),
    selection: Some(Color::from_hex(0x1a1f29)),
    line_highlight: Some(Color::from_hex(0x0d1017)),
    gutter: Some(Color::from_hex(0x626a73)),
    statusbar_bg: Some(Color::from_hex(0x0d1017)),
    statusbar_fg: Some(Color::from_hex(0x6c7380)),
    comment: Some(Color::from_hex(0x626a73)),
    keyword: Some(Color::from_hex(0xff8f40)),
    string: Some(Color::from_hex(0xc2d94c)),
    function: Some(Color::from_hex(0xffb454)),
    variable: Some(Color::from_hex(0xe6b450)),
    r#type: Some(Color::from_hex(0x59c2ff)),
    constant: Some(Color::from_hex(0xe6b673)),
    operator: Some(Color::from_hex(0xf29668)),
    tag: Some(Color::from_hex(0x39bae6)),
    error: Some(Color::from_hex(0xff3333)),
    warning: Some(Color::from_hex(0xff8f40)),
    info: Some(Color::from_hex(0x59c2ff)),
    success: Some(Color::from_hex(0xc2d94c)),
    red: Some(Color::from_hex(0xff3333)),
    orange: Some(Color::from_hex(0xff8f40)),
    yellow: Some(Color::from_hex(0xe6b450)),
    green: Some(Color::from_hex(0xc2d94c)),
    cyan: Some(Color::from_hex(0x95e6cb)),
    blue: Some(Color::from_hex(0x59c2ff)),
    purple: Some(Color::from_hex(0xd2a6ff)),
    magenta: Some(Color::from_hex(0xf07178)),
};

/// Ayu Mirage — the mirage variant.
///
/// Author: Ike Ku
/// Variant: Dark
/// Contrast: Normal
/// Source: ayu-colors
pub const MIRAGE: Theme = Theme {
    name: "Ayu Mirage",
    author: "Ike Ku",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x1f2430),
    fg: Color::from_hex(0xcbccc6),
    cursor: Some(Color::from_hex(0xffcc66)),
    selection: Some(Color::from_hex(0x33415e)),
    line_highlight: Some(Color::from_hex(0x191e2a)),
    gutter: Some(Color::from_hex(0x5c6773)),
    statusbar_bg: Some(Color::from_hex(0x191e2a)),
    statusbar_fg: Some(Color::from_hex(0x707a8c)),
    comment: Some(Color::from_hex(0x5c6773)),
    keyword: Some(Color::from_hex(0xffad66)),
    string: Some(Color::from_hex(0xd5ff80)),
    function: Some(Color::from_hex(0xffd580)),
    variable: Some(Color::from_hex(0xf29e74)),
    r#type: Some(Color::from_hex(0x73d0ff)),
    constant: Some(Color::from_hex(0xdfbfff)),
    operator: Some(Color::from_hex(0xf29e74)),
    tag: Some(Color::from_hex(0x5ccfe6)),
    error: Some(Color::from_hex(0xff3333)),
    warning: Some(Color::from_hex(0xffd580)),
    info: Some(Color::from_hex(0x73d0ff)),
    success: Some(Color::from_hex(0xd5ff80)),
    red: Some(Color::from_hex(0xff3333)),
    orange: Some(Color::from_hex(0xffad66)),
    yellow: Some(Color::from_hex(0xffd580)),
    green: Some(Color::from_hex(0xbae67e)),
    cyan: Some(Color::from_hex(0x5ccfe6)),
    blue: Some(Color::from_hex(0x73d0ff)),
    purple: Some(Color::from_hex(0xdfbfff)),
    magenta: Some(Color::from_hex(0xf07178)),
};

/// Ayu Light — the light variant.
///
/// Author: Ike Ku
/// Variant: Light
/// Contrast: Normal
/// Source: ayu-colors
pub const LIGHT: Theme = Theme {
    name: "Ayu Light",
    author: "Ike Ku",
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfafafa),
    fg: Color::from_hex(0x575f66),
    cursor: Some(Color::from_hex(0xff6a00)),
    selection: Some(Color::from_hex(0xe7e8e9)),
    line_highlight: Some(Color::from_hex(0xf0f0f0)),
    gutter: Some(Color::from_hex(0xabb0b6)),
    statusbar_bg: Some(Color::from_hex(0xf0f0f0)),
    statusbar_fg: Some(Color::from_hex(0x828c99)),
    comment: Some(Color::from_hex(0xabb0b6)),
    keyword: Some(Color::from_hex(0xf2ae49)),
    string: Some(Color::from_hex(0x86b300)),
    function: Some(Color::from_hex(0xf29718)),
    variable: Some(Color::from_hex(0xa37acc)),
    r#type: Some(Color::from_hex(0x399ee6)),
    constant: Some(Color::from_hex(0xf29718)),
    operator: Some(Color::from_hex(0xed9366)),
    tag: Some(Color::from_hex(0x55b4d4)),
    error: Some(Color::from_hex(0xf51818)),
    warning: Some(Color::from_hex(0xf2ae49)),
    info: Some(Color::from_hex(0x399ee6)),
    success: Some(Color::from_hex(0x86b300)),
    red: Some(Color::from_hex(0xf51818)),
    orange: Some(Color::from_hex(0xf29718)),
    yellow: Some(Color::from_hex(0xf2ae49)),
    green: Some(Color::from_hex(0x86b300)),
    cyan: Some(Color::from_hex(0x4cbf99)),
    blue: Some(Color::from_hex(0x399ee6)),
    purple: Some(Color::from_hex(0xa37acc)),
    magenta: Some(Color::from_hex(0xf07178)),
};
