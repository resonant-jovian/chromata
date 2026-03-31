//! Solarized color themes.
//!
//! A precision color scheme by Ethan Schoonover.
//! <https://github.com/altercation/solarized>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Solarized Dark — dark variant.
///
/// Author: Ethan Schoonover
/// Variant: Dark
/// Contrast: Normal
/// Source: <https://ethanschoonover.com/solarized/>
pub const DARK: Theme = Theme {
    name: Cow::Borrowed("Solarized Dark"),
    author: Cow::Borrowed("Ethan Schoonover"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x002b36),
    fg: Color::from_hex(0x839496),
    cursor: Some(Color::from_hex(0x839496)),
    selection: Some(Color::from_hex(0x073642)),
    line_highlight: Some(Color::from_hex(0x073642)),
    gutter: Some(Color::from_hex(0x586e75)),
    statusbar_bg: Some(Color::from_hex(0x073642)),
    statusbar_fg: Some(Color::from_hex(0x93a1a1)),
    comment: Some(Color::from_hex(0x586e75)),
    keyword: Some(Color::from_hex(0x859900)),
    string: Some(Color::from_hex(0x2aa198)),
    function: Some(Color::from_hex(0x268bd2)),
    variable: Some(Color::from_hex(0xb58900)),
    r#type: Some(Color::from_hex(0xcb4b16)),
    constant: Some(Color::from_hex(0xd33682)),
    operator: Some(Color::from_hex(0x839496)),
    tag: Some(Color::from_hex(0xb58900)),
    error: Some(Color::from_hex(0xdc322f)),
    warning: Some(Color::from_hex(0xb58900)),
    info: Some(Color::from_hex(0x268bd2)),
    success: Some(Color::from_hex(0x859900)),
    red: Some(Color::from_hex(0xdc322f)),
    orange: Some(Color::from_hex(0xcb4b16)),
    yellow: Some(Color::from_hex(0xb58900)),
    green: Some(Color::from_hex(0x859900)),
    cyan: Some(Color::from_hex(0x2aa198)),
    blue: Some(Color::from_hex(0x268bd2)),
    purple: Some(Color::from_hex(0x6c71c4)),
    magenta: Some(Color::from_hex(0xd33682)),
};

/// Solarized Light — light variant.
///
/// Author: Ethan Schoonover
/// Variant: Light
/// Contrast: Low
/// Source: <https://ethanschoonover.com/solarized/>
pub const LIGHT: Theme = Theme {
    name: Cow::Borrowed("Solarized Light"),
    author: Cow::Borrowed("Ethan Schoonover"),
    variant: Variant::Light,
    contrast: Contrast::Low,
    bg: Color::from_hex(0xfdf6e3),
    fg: Color::from_hex(0x657b83),
    cursor: Some(Color::from_hex(0x657b83)),
    selection: Some(Color::from_hex(0xeee8d5)),
    line_highlight: Some(Color::from_hex(0xeee8d5)),
    gutter: Some(Color::from_hex(0x93a1a1)),
    statusbar_bg: Some(Color::from_hex(0xeee8d5)),
    statusbar_fg: Some(Color::from_hex(0x586e75)),
    comment: Some(Color::from_hex(0x586e75)),
    keyword: Some(Color::from_hex(0x859900)),
    string: Some(Color::from_hex(0x2aa198)),
    function: Some(Color::from_hex(0x268bd2)),
    variable: Some(Color::from_hex(0xb58900)),
    r#type: Some(Color::from_hex(0xcb4b16)),
    constant: Some(Color::from_hex(0xd33682)),
    operator: Some(Color::from_hex(0x657b83)),
    tag: Some(Color::from_hex(0xb58900)),
    error: Some(Color::from_hex(0xdc322f)),
    warning: Some(Color::from_hex(0xb58900)),
    info: Some(Color::from_hex(0x268bd2)),
    success: Some(Color::from_hex(0x859900)),
    red: Some(Color::from_hex(0xdc322f)),
    orange: Some(Color::from_hex(0xcb4b16)),
    yellow: Some(Color::from_hex(0xb58900)),
    green: Some(Color::from_hex(0x859900)),
    cyan: Some(Color::from_hex(0x2aa198)),
    blue: Some(Color::from_hex(0x268bd2)),
    purple: Some(Color::from_hex(0x6c71c4)),
    magenta: Some(Color::from_hex(0xd33682)),
};
