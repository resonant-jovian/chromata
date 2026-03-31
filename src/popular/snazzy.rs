//! Snazzy color theme.
//!
//! An elegant color scheme by sindresorhus.
//! <https://github.com/sindresorhus/hyper-snazzy>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Snazzy — vibrant dark theme.
///
/// Author: sindresorhus
/// Variant: Dark
/// Contrast: High
/// Source: hyper-snazzy
pub const THEME: Theme = Theme {
    name: Cow::Borrowed("Snazzy"),
    author: Cow::Borrowed("sindresorhus"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x282a36),
    fg: Color::from_hex(0xeff0eb),
    cursor: Some(Color::from_hex(0x97979b)),
    selection: Some(Color::from_hex(0x3e4452)),
    line_highlight: Some(Color::from_hex(0x34353e)),
    gutter: Some(Color::from_hex(0x78787e)),
    statusbar_bg: Some(Color::from_hex(0x34353e)),
    statusbar_fg: Some(Color::from_hex(0xa2a2a8)),
    comment: Some(Color::from_hex(0x78787e)),
    keyword: Some(Color::from_hex(0xff6ac1)),
    string: Some(Color::from_hex(0x5af78e)),
    function: Some(Color::from_hex(0x57c7ff)),
    variable: Some(Color::from_hex(0xf3f99d)),
    r#type: Some(Color::from_hex(0x9aedfe)),
    constant: Some(Color::from_hex(0xff9f43)),
    operator: Some(Color::from_hex(0xff6ac1)),
    tag: Some(Color::from_hex(0xff5c57)),
    error: Some(Color::from_hex(0xff5c57)),
    warning: Some(Color::from_hex(0xf3f99d)),
    info: Some(Color::from_hex(0x57c7ff)),
    success: Some(Color::from_hex(0x5af78e)),
    red: Some(Color::from_hex(0xff5c57)),
    orange: Some(Color::from_hex(0xff9f43)),
    yellow: Some(Color::from_hex(0xf3f99d)),
    green: Some(Color::from_hex(0x5af78e)),
    cyan: Some(Color::from_hex(0x9aedfe)),
    blue: Some(Color::from_hex(0x57c7ff)),
    purple: Some(Color::from_hex(0xff6ac1)),
    magenta: Some(Color::from_hex(0xff5c57)),
};
