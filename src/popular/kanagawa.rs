//! Kanagawa color themes.
//!
//! A dark color scheme inspired by the famous painting by Katsushika Hokusai.
//! <https://github.com/rebelot/kanagawa.nvim>

use crate::{Color, Contrast, Theme, Variant};

/// Kanagawa Wave — the original dark variant.
///
/// Author: rebelot
/// Variant: Dark
/// Contrast: High
/// Source: rebelot/kanagawa.nvim
pub const WAVE: Theme = Theme {
    name: "Kanagawa Wave",
    author: "rebelot",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x1f1f28),
    fg: Color::from_hex(0xdcd7ba),
    cursor: Some(Color::from_hex(0xc8c093)),
    selection: Some(Color::from_hex(0x2d4f67)),
    line_highlight: Some(Color::from_hex(0x2a2a37)),
    gutter: Some(Color::from_hex(0x727169)),
    statusbar_bg: Some(Color::from_hex(0x16161d)),
    statusbar_fg: Some(Color::from_hex(0xc8c093)),
    comment: Some(Color::from_hex(0x727169)),
    keyword: Some(Color::from_hex(0x957fb8)),
    string: Some(Color::from_hex(0x98bb6c)),
    function: Some(Color::from_hex(0x7e9cd8)),
    variable: Some(Color::from_hex(0xe6c384)),
    r#type: Some(Color::from_hex(0x7fb4ca)),
    constant: Some(Color::from_hex(0xd27e99)),
    operator: Some(Color::from_hex(0xc0a36e)),
    tag: Some(Color::from_hex(0xe46876)),
    error: Some(Color::from_hex(0xe82424)),
    warning: Some(Color::from_hex(0xe6c384)),
    info: Some(Color::from_hex(0x7fb4ca)),
    success: Some(Color::from_hex(0x98bb6c)),
    red: Some(Color::from_hex(0xc34043)),
    orange: Some(Color::from_hex(0xffa066)),
    yellow: Some(Color::from_hex(0xe6c384)),
    green: Some(Color::from_hex(0x98bb6c)),
    cyan: Some(Color::from_hex(0x7aa89f)),
    blue: Some(Color::from_hex(0x7e9cd8)),
    purple: Some(Color::from_hex(0x957fb8)),
    magenta: Some(Color::from_hex(0xd27e99)),
};

/// Kanagawa Dragon — the dragon dark variant.
///
/// Author: rebelot
/// Variant: Dark
/// Contrast: High
/// Source: rebelot/kanagawa.nvim
pub const DRAGON: Theme = Theme {
    name: "Kanagawa Dragon",
    author: "rebelot",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x181616),
    fg: Color::from_hex(0xc5c9c5),
    cursor: Some(Color::from_hex(0xc8c093)),
    selection: Some(Color::from_hex(0x2d4f67)),
    line_highlight: Some(Color::from_hex(0x1d1c19)),
    gutter: Some(Color::from_hex(0xa6a69c)),
    statusbar_bg: Some(Color::from_hex(0x12120f)),
    statusbar_fg: Some(Color::from_hex(0xc5c9c5)),
    comment: Some(Color::from_hex(0xa6a69c)),
    keyword: Some(Color::from_hex(0x8992a7)),
    string: Some(Color::from_hex(0x87a987)),
    function: Some(Color::from_hex(0x8ba4b0)),
    variable: Some(Color::from_hex(0xc4b28a)),
    r#type: Some(Color::from_hex(0x8ea4a2)),
    constant: Some(Color::from_hex(0xb6927b)),
    operator: Some(Color::from_hex(0xc4b28a)),
    tag: Some(Color::from_hex(0xc4746e)),
    error: Some(Color::from_hex(0xc4746e)),
    warning: Some(Color::from_hex(0xc4b28a)),
    info: Some(Color::from_hex(0x8ba4b0)),
    success: Some(Color::from_hex(0x87a987)),
    red: Some(Color::from_hex(0xc4746e)),
    orange: Some(Color::from_hex(0xb6927b)),
    yellow: Some(Color::from_hex(0xc4b28a)),
    green: Some(Color::from_hex(0x87a987)),
    cyan: Some(Color::from_hex(0x8ea4a2)),
    blue: Some(Color::from_hex(0x8ba4b0)),
    purple: Some(Color::from_hex(0x8992a7)),
    magenta: Some(Color::from_hex(0xa292a3)),
};
