//! One Dark and One Light color themes.
//!
//! The iconic editor themes from Atom by GitHub.
//! <https://github.com/atom/atom/tree/master/packages/one-dark-syntax>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// One Dark — dark variant.
///
/// Author: Atom (GitHub)
/// Variant: Dark
/// Contrast: Normal
/// Source: <https://github.com/atom/atom/tree/master/packages/one-dark-syntax>
pub const THEME: Theme = Theme {
    name: Cow::Borrowed("One Dark"),
    author: Cow::Borrowed("Atom (GitHub)"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x282c34),
    fg: Color::from_hex(0xabb2bf),
    cursor: Some(Color::from_hex(0xabb2bf)),
    selection: Some(Color::from_hex(0x3e4451)),
    line_highlight: Some(Color::from_hex(0x2c313c)),
    gutter: Some(Color::from_hex(0x4b5263)),
    statusbar_bg: Some(Color::from_hex(0x21252b)),
    statusbar_fg: Some(Color::from_hex(0x9da5b4)),
    comment: Some(Color::from_hex(0x5c6370)),
    keyword: Some(Color::from_hex(0xc678dd)),
    string: Some(Color::from_hex(0x98c379)),
    function: Some(Color::from_hex(0x61afef)),
    variable: Some(Color::from_hex(0xe06c75)),
    r#type: Some(Color::from_hex(0xe5c07b)),
    constant: Some(Color::from_hex(0xd19a66)),
    operator: Some(Color::from_hex(0x56b6c2)),
    tag: Some(Color::from_hex(0xe06c75)),
    error: Some(Color::from_hex(0xe06c75)),
    warning: Some(Color::from_hex(0xe5c07b)),
    info: Some(Color::from_hex(0x61afef)),
    success: Some(Color::from_hex(0x98c379)),
    red: Some(Color::from_hex(0xe06c75)),
    orange: Some(Color::from_hex(0xd19a66)),
    yellow: Some(Color::from_hex(0xe5c07b)),
    green: Some(Color::from_hex(0x98c379)),
    cyan: Some(Color::from_hex(0x56b6c2)),
    blue: Some(Color::from_hex(0x61afef)),
    purple: Some(Color::from_hex(0xc678dd)),
    magenta: Some(Color::from_hex(0xbe5046)),
};

/// One Light — light variant.
///
/// Author: Atom (GitHub)
/// Variant: Light
/// Contrast: High
/// Source: <https://github.com/atom/atom/tree/master/packages/one-light-syntax>
pub const LIGHT: Theme = Theme {
    name: Cow::Borrowed("One Light"),
    author: Cow::Borrowed("Atom (GitHub)"),
    variant: Variant::Light,
    contrast: Contrast::High,
    bg: Color::from_hex(0xfafafa),
    fg: Color::from_hex(0x383a42),
    cursor: Some(Color::from_hex(0x383a42)),
    selection: Some(Color::from_hex(0xe2e2e3)),
    line_highlight: Some(Color::from_hex(0xf0f0f0)),
    gutter: Some(Color::from_hex(0xa0a1a7)),
    statusbar_bg: Some(Color::from_hex(0xf0f0f0)),
    statusbar_fg: Some(Color::from_hex(0x696c77)),
    comment: Some(Color::from_hex(0xa0a1a7)),
    keyword: Some(Color::from_hex(0xa626a4)),
    string: Some(Color::from_hex(0x50a14f)),
    function: Some(Color::from_hex(0x4078f2)),
    variable: Some(Color::from_hex(0xe45649)),
    r#type: Some(Color::from_hex(0xc18401)),
    constant: Some(Color::from_hex(0x986801)),
    operator: Some(Color::from_hex(0x0184bc)),
    tag: Some(Color::from_hex(0xe45649)),
    error: Some(Color::from_hex(0xe45649)),
    warning: Some(Color::from_hex(0xc18401)),
    info: Some(Color::from_hex(0x4078f2)),
    success: Some(Color::from_hex(0x50a14f)),
    red: Some(Color::from_hex(0xe45649)),
    orange: Some(Color::from_hex(0x986801)),
    yellow: Some(Color::from_hex(0xc18401)),
    green: Some(Color::from_hex(0x50a14f)),
    cyan: Some(Color::from_hex(0x0184bc)),
    blue: Some(Color::from_hex(0x4078f2)),
    purple: Some(Color::from_hex(0xa626a4)),
    magenta: Some(Color::from_hex(0xca1243)),
};
