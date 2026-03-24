//! Everforest color themes.
//!
//! A comfortable and pleasant green forest color scheme by sainnhe.
//! <https://github.com/sainnhe/everforest>

use crate::{Color, Contrast, Theme, Variant};

/// Everforest Dark — the dark variant.
///
/// Author: sainnhe
/// Variant: Dark
/// Contrast: Normal
/// Source: sainnhe/everforest
pub const DARK: Theme = Theme {
    name: "Everforest Dark",
    author: "sainnhe",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x2d353b),
    fg: Color::from_hex(0xd3c6aa),
    cursor: Some(Color::from_hex(0xd3c6aa)),
    selection: Some(Color::from_hex(0x3d484d)),
    line_highlight: Some(Color::from_hex(0x343f44)),
    gutter: Some(Color::from_hex(0x859289)),
    statusbar_bg: Some(Color::from_hex(0x343f44)),
    statusbar_fg: Some(Color::from_hex(0x9da9a0)),
    comment: Some(Color::from_hex(0x859289)),
    keyword: Some(Color::from_hex(0xe67e80)),
    string: Some(Color::from_hex(0xa7c080)),
    function: Some(Color::from_hex(0x7fbbb3)),
    variable: Some(Color::from_hex(0xd699b6)),
    r#type: Some(Color::from_hex(0xdbbc7f)),
    constant: Some(Color::from_hex(0xe69875)),
    operator: Some(Color::from_hex(0xd699b6)),
    tag: Some(Color::from_hex(0xe67e80)),
    error: Some(Color::from_hex(0xe67e80)),
    warning: Some(Color::from_hex(0xdbbc7f)),
    info: Some(Color::from_hex(0x7fbbb3)),
    success: Some(Color::from_hex(0xa7c080)),
    red: Some(Color::from_hex(0xe67e80)),
    orange: Some(Color::from_hex(0xe69875)),
    yellow: Some(Color::from_hex(0xdbbc7f)),
    green: Some(Color::from_hex(0xa7c080)),
    cyan: Some(Color::from_hex(0x83c092)),
    blue: Some(Color::from_hex(0x7fbbb3)),
    purple: Some(Color::from_hex(0xd699b6)),
    magenta: Some(Color::from_hex(0xe67e80)),
};

/// Everforest Light — the light variant.
///
/// Author: sainnhe
/// Variant: Light
/// Contrast: Normal
/// Source: sainnhe/everforest
pub const LIGHT: Theme = Theme {
    name: "Everforest Light",
    author: "sainnhe",
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfdf6e3),
    fg: Color::from_hex(0x5c6a72),
    cursor: Some(Color::from_hex(0x5c6a72)),
    selection: Some(Color::from_hex(0xe6e2cc)),
    line_highlight: Some(Color::from_hex(0xf4f0d9)),
    gutter: Some(Color::from_hex(0xa6b0a0)),
    statusbar_bg: Some(Color::from_hex(0xf4f0d9)),
    statusbar_fg: Some(Color::from_hex(0x829181)),
    comment: Some(Color::from_hex(0xa6b0a0)),
    keyword: Some(Color::from_hex(0xf85552)),
    string: Some(Color::from_hex(0x8da101)),
    function: Some(Color::from_hex(0x3a94c5)),
    variable: Some(Color::from_hex(0xdf69ba)),
    r#type: Some(Color::from_hex(0xdfa000)),
    constant: Some(Color::from_hex(0xf57d26)),
    operator: Some(Color::from_hex(0xdf69ba)),
    tag: Some(Color::from_hex(0xf85552)),
    error: Some(Color::from_hex(0xf85552)),
    warning: Some(Color::from_hex(0xdfa000)),
    info: Some(Color::from_hex(0x3a94c5)),
    success: Some(Color::from_hex(0x8da101)),
    red: Some(Color::from_hex(0xf85552)),
    orange: Some(Color::from_hex(0xf57d26)),
    yellow: Some(Color::from_hex(0xdfa000)),
    green: Some(Color::from_hex(0x8da101)),
    cyan: Some(Color::from_hex(0x35a77c)),
    blue: Some(Color::from_hex(0x3a94c5)),
    purple: Some(Color::from_hex(0xdf69ba)),
    magenta: Some(Color::from_hex(0xf85552)),
};
