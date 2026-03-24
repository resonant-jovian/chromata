//! Moonlight color theme.
//!
//! A dark theme with cool tones inspired by moonlight by atomiks.
//! <https://github.com/atomiks/moonlight-vscode-theme>

use crate::{Color, Contrast, Theme, Variant};

/// Moonlight — dark variant.
///
/// Author: atomiks
/// Variant: Dark
/// Contrast: High
/// Source: moonlight-vscode-theme
pub const THEME: Theme = Theme {
    name: "Moonlight",
    author: "atomiks",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x222436),
    fg: Color::from_hex(0xc8d3f5),
    cursor: Some(Color::from_hex(0xc8d3f5)),
    selection: Some(Color::from_hex(0x2f334d)),
    line_highlight: Some(Color::from_hex(0x2f334d)),
    gutter: Some(Color::from_hex(0x636da6)),
    statusbar_bg: Some(Color::from_hex(0x1e2030)),
    statusbar_fg: Some(Color::from_hex(0x828bb8)),
    comment: Some(Color::from_hex(0x636da6)),
    keyword: Some(Color::from_hex(0xc099ff)),
    string: Some(Color::from_hex(0xc3e88d)),
    function: Some(Color::from_hex(0x82aaff)),
    variable: Some(Color::from_hex(0xff966c)),
    r#type: Some(Color::from_hex(0xffc777)),
    constant: Some(Color::from_hex(0xff98a4)),
    operator: Some(Color::from_hex(0x86e1fc)),
    tag: Some(Color::from_hex(0xff757f)),
    error: Some(Color::from_hex(0xff757f)),
    warning: Some(Color::from_hex(0xffc777)),
    info: Some(Color::from_hex(0x82aaff)),
    success: Some(Color::from_hex(0xc3e88d)),
    red: Some(Color::from_hex(0xff757f)),
    orange: Some(Color::from_hex(0xff966c)),
    yellow: Some(Color::from_hex(0xffc777)),
    green: Some(Color::from_hex(0xc3e88d)),
    cyan: Some(Color::from_hex(0x86e1fc)),
    blue: Some(Color::from_hex(0x82aaff)),
    purple: Some(Color::from_hex(0xc099ff)),
    magenta: Some(Color::from_hex(0xff757f)),
};
