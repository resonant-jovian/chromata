//! Dracula color theme.
//!
//! A dark theme for all the things by Zeno Rocha.
//! <https://github.com/dracula/dracula-theme>

use crate::{Color, Contrast, Theme, Variant};

/// Dracula — dark theme.
///
/// Author: Zeno Rocha
/// Variant: Dark
/// Contrast: High
/// Source: <https://draculatheme.com/>
pub const THEME: Theme = Theme {
    name: "Dracula",
    author: "Zeno Rocha",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x282a36),
    fg: Color::from_hex(0xf8f8f2),
    cursor: Some(Color::from_hex(0xf8f8f2)),
    selection: Some(Color::from_hex(0x44475a)),
    line_highlight: Some(Color::from_hex(0x44475a)),
    gutter: Some(Color::from_hex(0x6272a4)),
    statusbar_bg: Some(Color::from_hex(0x44475a)),
    statusbar_fg: Some(Color::from_hex(0xf8f8f2)),
    comment: Some(Color::from_hex(0x6272a4)),
    keyword: Some(Color::from_hex(0xff79c6)),
    string: Some(Color::from_hex(0xf1fa8c)),
    function: Some(Color::from_hex(0x50fa7b)),
    variable: Some(Color::from_hex(0xffb86c)),
    r#type: Some(Color::from_hex(0x8be9fd)),
    constant: Some(Color::from_hex(0xbd93f9)),
    operator: Some(Color::from_hex(0xff79c6)),
    tag: Some(Color::from_hex(0xff79c6)),
    error: Some(Color::from_hex(0xff5555)),
    warning: Some(Color::from_hex(0xf1fa8c)),
    info: Some(Color::from_hex(0x8be9fd)),
    success: Some(Color::from_hex(0x50fa7b)),
    red: Some(Color::from_hex(0xff5555)),
    orange: Some(Color::from_hex(0xffb86c)),
    yellow: Some(Color::from_hex(0xf1fa8c)),
    green: Some(Color::from_hex(0x50fa7b)),
    cyan: Some(Color::from_hex(0x8be9fd)),
    blue: Some(Color::from_hex(0x6272a4)),
    purple: Some(Color::from_hex(0xbd93f9)),
    magenta: Some(Color::from_hex(0xff79c6)),
};
