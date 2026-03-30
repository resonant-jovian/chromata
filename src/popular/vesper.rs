//! Vesper color theme.
//!
//! A minimal dark theme by raunofreiberg.
//! <https://github.com/raunofreiberg/vesper>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Vesper — minimal dark theme.
///
/// Author: raunofreiberg
/// Variant: Dark
/// Contrast: Normal
/// Source: vesper
pub const THEME: Theme = Theme {
    name: Cow::Borrowed("Vesper"),
    author: Cow::Borrowed("raunofreiberg"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x101010),
    fg: Color::from_hex(0xb0b0b0),
    cursor: Some(Color::from_hex(0xb0b0b0)),
    selection: Some(Color::from_hex(0x1e1e1e)),
    line_highlight: Some(Color::from_hex(0x181818)),
    gutter: Some(Color::from_hex(0x505050)),
    statusbar_bg: Some(Color::from_hex(0x181818)),
    statusbar_fg: Some(Color::from_hex(0x707070)),
    comment: Some(Color::from_hex(0x505050)),
    keyword: Some(Color::from_hex(0xffc799)),
    string: Some(Color::from_hex(0x99ffe4)),
    function: Some(Color::from_hex(0xa0a0a0)),
    variable: Some(Color::from_hex(0xffffff)),
    r#type: Some(Color::from_hex(0xffc799)),
    constant: Some(Color::from_hex(0xffc799)),
    operator: Some(Color::from_hex(0xa0a0a0)),
    tag: Some(Color::from_hex(0xffc799)),
    error: Some(Color::from_hex(0xff6464)),
    warning: Some(Color::from_hex(0xffc799)),
    info: Some(Color::from_hex(0x60a0a0)),
    success: Some(Color::from_hex(0x99ffe4)),
    red: Some(Color::from_hex(0xff6464)),
    orange: Some(Color::from_hex(0xffc799)),
    yellow: Some(Color::from_hex(0xffc799)),
    green: Some(Color::from_hex(0x99ffe4)),
    cyan: Some(Color::from_hex(0x60a0a0)),
    blue: Some(Color::from_hex(0x60a0a0)),
    purple: Some(Color::from_hex(0xddbfff)),
    magenta: Some(Color::from_hex(0xff6464)),
};
