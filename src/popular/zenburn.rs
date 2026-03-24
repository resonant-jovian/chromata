//! Zenburn color theme.
//!
//! A low-contrast dark color scheme by Jani Nurminen.
//! <https://kippura.org/zenburnpage/>

use crate::{Color, Contrast, Theme, Variant};

/// Zenburn — low-contrast dark theme.
///
/// Author: Jani Nurminen
/// Variant: Dark
/// Contrast: Normal
/// Source: zenburn
pub const THEME: Theme = Theme {
    name: "Zenburn",
    author: "Jani Nurminen",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x3f3f3f),
    fg: Color::from_hex(0xdcdccc),
    cursor: Some(Color::from_hex(0xdcdccc)),
    selection: Some(Color::from_hex(0x2b2b2b)),
    line_highlight: Some(Color::from_hex(0x484848)),
    gutter: Some(Color::from_hex(0x6f6f6f)),
    statusbar_bg: Some(Color::from_hex(0x2b2b2b)),
    statusbar_fg: Some(Color::from_hex(0xdcdccc)),
    comment: Some(Color::from_hex(0x7f9f7f)),
    keyword: Some(Color::from_hex(0xf0dfaf)),
    string: Some(Color::from_hex(0xcc9393)),
    function: Some(Color::from_hex(0xefef8f)),
    variable: Some(Color::from_hex(0xdfaf8f)),
    r#type: Some(Color::from_hex(0x8cd0d3)),
    constant: Some(Color::from_hex(0xdca3a3)),
    operator: Some(Color::from_hex(0xf0efd0)),
    tag: Some(Color::from_hex(0xdfaf8f)),
    error: Some(Color::from_hex(0xe37170)),
    warning: Some(Color::from_hex(0xdfaf8f)),
    info: Some(Color::from_hex(0x8cd0d3)),
    success: Some(Color::from_hex(0x7f9f7f)),
    red: Some(Color::from_hex(0xcc9393)),
    orange: Some(Color::from_hex(0xdfaf8f)),
    yellow: Some(Color::from_hex(0xf0dfaf)),
    green: Some(Color::from_hex(0x7f9f7f)),
    cyan: Some(Color::from_hex(0x93e0e3)),
    blue: Some(Color::from_hex(0x8cd0d3)),
    purple: Some(Color::from_hex(0xdc8cc3)),
    magenta: Some(Color::from_hex(0xcc9393)),
};
