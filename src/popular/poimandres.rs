//! Poimandres color theme.
//!
//! A minimal, dark color scheme by drcmda.
//! <https://github.com/drcmda/poimandres-theme>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Poimandres — minimal dark theme.
///
/// Author: drcmda
/// Variant: Dark
/// Contrast: Normal
/// Source: poimandres-theme
pub const THEME: Theme = Theme {
    name: Cow::Borrowed("Poimandres"),
    author: Cow::Borrowed("drcmda"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x1b1e28),
    fg: Color::from_hex(0xa6accd),
    cursor: Some(Color::from_hex(0xa6accd)),
    selection: Some(Color::from_hex(0x303340)),
    line_highlight: Some(Color::from_hex(0x252b37)),
    gutter: Some(Color::from_hex(0x506477)),
    statusbar_bg: Some(Color::from_hex(0x171922)),
    statusbar_fg: Some(Color::from_hex(0x767c9d)),
    comment: Some(Color::from_hex(0x767c9d)),
    keyword: Some(Color::from_hex(0x5de4c7)),
    string: Some(Color::from_hex(0x5de4c7)),
    function: Some(Color::from_hex(0xadd7ff)),
    variable: Some(Color::from_hex(0x91b4d5)),
    r#type: Some(Color::from_hex(0xfcc5e9)),
    constant: Some(Color::from_hex(0xd0679d)),
    operator: Some(Color::from_hex(0x91b4d5)),
    tag: Some(Color::from_hex(0x5de4c7)),
    error: Some(Color::from_hex(0xd0679d)),
    warning: Some(Color::from_hex(0xfffac2)),
    info: Some(Color::from_hex(0xadd7ff)),
    success: Some(Color::from_hex(0x5de4c7)),
    red: Some(Color::from_hex(0xd0679d)),
    orange: Some(Color::from_hex(0xfcc5e9)),
    yellow: Some(Color::from_hex(0xfffac2)),
    green: Some(Color::from_hex(0x5de4c7)),
    cyan: Some(Color::from_hex(0x89ddff)),
    blue: Some(Color::from_hex(0xadd7ff)),
    purple: Some(Color::from_hex(0xa6accd)),
    magenta: Some(Color::from_hex(0xd0679d)),
};
