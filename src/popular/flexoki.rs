//! Flexoki color themes.
//!
//! An inky color scheme by Steph Ango.
//! <https://github.com/kepano/flexoki>

use crate::{Color, Contrast, Theme, Variant};

/// Flexoki Dark — dark variant.
///
/// Author: Steph Ango
/// Variant: Dark
/// Contrast: High
/// Source: flexoki
pub const DARK: Theme = Theme {
    name: "Flexoki Dark",
    author: "Steph Ango",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x100f0f),
    fg: Color::from_hex(0xcecdc3),
    cursor: Some(Color::from_hex(0xcecdc3)),
    selection: Some(Color::from_hex(0x282726)),
    line_highlight: Some(Color::from_hex(0x1c1b1a)),
    gutter: Some(Color::from_hex(0x575653)),
    statusbar_bg: Some(Color::from_hex(0x1c1b1a)),
    statusbar_fg: Some(Color::from_hex(0x878580)),
    comment: Some(Color::from_hex(0x878580)),
    keyword: Some(Color::from_hex(0xce5d97)),
    string: Some(Color::from_hex(0x879a39)),
    function: Some(Color::from_hex(0x4385be)),
    variable: Some(Color::from_hex(0xda702c)),
    r#type: Some(Color::from_hex(0xd0a215)),
    constant: Some(Color::from_hex(0x8b7ec8)),
    operator: Some(Color::from_hex(0x3aa99f)),
    tag: Some(Color::from_hex(0xd14d41)),
    error: Some(Color::from_hex(0xd14d41)),
    warning: Some(Color::from_hex(0xd0a215)),
    info: Some(Color::from_hex(0x4385be)),
    success: Some(Color::from_hex(0x879a39)),
    red: Some(Color::from_hex(0xd14d41)),
    orange: Some(Color::from_hex(0xda702c)),
    yellow: Some(Color::from_hex(0xd0a215)),
    green: Some(Color::from_hex(0x879a39)),
    cyan: Some(Color::from_hex(0x3aa99f)),
    blue: Some(Color::from_hex(0x4385be)),
    purple: Some(Color::from_hex(0x8b7ec8)),
    magenta: Some(Color::from_hex(0xce5d97)),
};

/// Flexoki Light — light variant.
///
/// Author: Steph Ango
/// Variant: Light
/// Contrast: High
/// Source: flexoki
pub const LIGHT: Theme = Theme {
    name: "Flexoki Light",
    author: "Steph Ango",
    variant: Variant::Light,
    contrast: Contrast::High,
    bg: Color::from_hex(0xfffcf0),
    fg: Color::from_hex(0x100f0f),
    cursor: Some(Color::from_hex(0x100f0f)),
    selection: Some(Color::from_hex(0xe6e4d9)),
    line_highlight: Some(Color::from_hex(0xf2f0e5)),
    gutter: Some(Color::from_hex(0xb7b5ac)),
    statusbar_bg: Some(Color::from_hex(0xf2f0e5)),
    statusbar_fg: Some(Color::from_hex(0x6f6e69)),
    comment: Some(Color::from_hex(0x878580)),
    keyword: Some(Color::from_hex(0xce5d97)),
    string: Some(Color::from_hex(0x879a39)),
    function: Some(Color::from_hex(0x4385be)),
    variable: Some(Color::from_hex(0xda702c)),
    r#type: Some(Color::from_hex(0xd0a215)),
    constant: Some(Color::from_hex(0x8b7ec8)),
    operator: Some(Color::from_hex(0x3aa99f)),
    tag: Some(Color::from_hex(0xd14d41)),
    error: Some(Color::from_hex(0xd14d41)),
    warning: Some(Color::from_hex(0xd0a215)),
    info: Some(Color::from_hex(0x4385be)),
    success: Some(Color::from_hex(0x879a39)),
    red: Some(Color::from_hex(0xd14d41)),
    orange: Some(Color::from_hex(0xda702c)),
    yellow: Some(Color::from_hex(0xd0a215)),
    green: Some(Color::from_hex(0x879a39)),
    cyan: Some(Color::from_hex(0x3aa99f)),
    blue: Some(Color::from_hex(0x4385be)),
    purple: Some(Color::from_hex(0x8b7ec8)),
    magenta: Some(Color::from_hex(0xce5d97)),
};
