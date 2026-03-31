//! Material color themes.
//!
//! The most epic theme for your editor by Mattia Astorino.
//! <https://github.com/material-theme/vsc-material-theme>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Material Theme — the default dark variant.
///
/// Author: Mattia Astorino
/// Variant: Dark
/// Contrast: High
/// Source: vsc-material-theme
pub const THEME: Theme = Theme {
    name: Cow::Borrowed("Material Theme"),
    author: Cow::Borrowed("Mattia Astorino"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x263238),
    fg: Color::from_hex(0xeeffff),
    cursor: Some(Color::from_hex(0xffcc00)),
    selection: Some(Color::from_hex(0x2c3b41)),
    line_highlight: Some(Color::from_hex(0x2c3b41)),
    gutter: Some(Color::from_hex(0x37474f)),
    statusbar_bg: Some(Color::from_hex(0x1e272c)),
    statusbar_fg: Some(Color::from_hex(0x607d8b)),
    comment: Some(Color::from_hex(0x546e7a)),
    keyword: Some(Color::from_hex(0xc792ea)),
    string: Some(Color::from_hex(0xc3e88d)),
    function: Some(Color::from_hex(0x82aaff)),
    variable: Some(Color::from_hex(0xf07178)),
    r#type: Some(Color::from_hex(0xffcb6b)),
    constant: Some(Color::from_hex(0xf78c6c)),
    operator: Some(Color::from_hex(0x89ddff)),
    tag: Some(Color::from_hex(0xf07178)),
    error: Some(Color::from_hex(0xff5370)),
    warning: Some(Color::from_hex(0xffcb6b)),
    info: Some(Color::from_hex(0x82aaff)),
    success: Some(Color::from_hex(0xc3e88d)),
    red: Some(Color::from_hex(0xff5370)),
    orange: Some(Color::from_hex(0xf78c6c)),
    yellow: Some(Color::from_hex(0xffcb6b)),
    green: Some(Color::from_hex(0xc3e88d)),
    cyan: Some(Color::from_hex(0x89ddff)),
    blue: Some(Color::from_hex(0x82aaff)),
    purple: Some(Color::from_hex(0xc792ea)),
    magenta: Some(Color::from_hex(0xff5370)),
};

/// Material Darker — the darker variant.
///
/// Author: Mattia Astorino
/// Variant: Dark
/// Contrast: High
/// Source: vsc-material-theme
pub const DARKER: Theme = Theme {
    name: Cow::Borrowed("Material Darker"),
    author: Cow::Borrowed("Mattia Astorino"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x212121),
    fg: Color::from_hex(0xeeffff),
    cursor: Some(Color::from_hex(0xffcc00)),
    selection: Some(Color::from_hex(0x2c2c2c)),
    line_highlight: Some(Color::from_hex(0x2c2c2c)),
    gutter: Some(Color::from_hex(0x424242)),
    statusbar_bg: Some(Color::from_hex(0x1a1a1a)),
    statusbar_fg: Some(Color::from_hex(0x616161)),
    comment: Some(Color::from_hex(0x545454)),
    keyword: Some(Color::from_hex(0xc792ea)),
    string: Some(Color::from_hex(0xc3e88d)),
    function: Some(Color::from_hex(0x82aaff)),
    variable: Some(Color::from_hex(0xf07178)),
    r#type: Some(Color::from_hex(0xffcb6b)),
    constant: Some(Color::from_hex(0xf78c6c)),
    operator: Some(Color::from_hex(0x89ddff)),
    tag: Some(Color::from_hex(0xf07178)),
    error: Some(Color::from_hex(0xff5370)),
    warning: Some(Color::from_hex(0xffcb6b)),
    info: Some(Color::from_hex(0x82aaff)),
    success: Some(Color::from_hex(0xc3e88d)),
    red: Some(Color::from_hex(0xff5370)),
    orange: Some(Color::from_hex(0xf78c6c)),
    yellow: Some(Color::from_hex(0xffcb6b)),
    green: Some(Color::from_hex(0xc3e88d)),
    cyan: Some(Color::from_hex(0x89ddff)),
    blue: Some(Color::from_hex(0x82aaff)),
    purple: Some(Color::from_hex(0xc792ea)),
    magenta: Some(Color::from_hex(0xff5370)),
};

/// Material Palenight — the palenight variant.
///
/// Author: Mattia Astorino
/// Variant: Dark
/// Contrast: Normal
/// Source: vsc-material-theme
pub const PALENIGHT: Theme = Theme {
    name: Cow::Borrowed("Material Palenight"),
    author: Cow::Borrowed("Mattia Astorino"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x292d3e),
    fg: Color::from_hex(0xa6accd),
    cursor: Some(Color::from_hex(0xffcc00)),
    selection: Some(Color::from_hex(0x2f3344)),
    line_highlight: Some(Color::from_hex(0x2f3344)),
    gutter: Some(Color::from_hex(0x3a3f58)),
    statusbar_bg: Some(Color::from_hex(0x202331)),
    statusbar_fg: Some(Color::from_hex(0x676e95)),
    comment: Some(Color::from_hex(0x676e95)),
    keyword: Some(Color::from_hex(0xc792ea)),
    string: Some(Color::from_hex(0xc3e88d)),
    function: Some(Color::from_hex(0x82aaff)),
    variable: Some(Color::from_hex(0xf07178)),
    r#type: Some(Color::from_hex(0xffcb6b)),
    constant: Some(Color::from_hex(0xf78c6c)),
    operator: Some(Color::from_hex(0x89ddff)),
    tag: Some(Color::from_hex(0xf07178)),
    error: Some(Color::from_hex(0xff5370)),
    warning: Some(Color::from_hex(0xffcb6b)),
    info: Some(Color::from_hex(0x82aaff)),
    success: Some(Color::from_hex(0xc3e88d)),
    red: Some(Color::from_hex(0xff5370)),
    orange: Some(Color::from_hex(0xf78c6c)),
    yellow: Some(Color::from_hex(0xffcb6b)),
    green: Some(Color::from_hex(0xc3e88d)),
    cyan: Some(Color::from_hex(0x89ddff)),
    blue: Some(Color::from_hex(0x82aaff)),
    purple: Some(Color::from_hex(0xc792ea)),
    magenta: Some(Color::from_hex(0xff5370)),
};

/// Material Lighter — the light variant.
///
/// Author: Mattia Astorino
/// Variant: Light
/// Contrast: Normal
/// Source: vsc-material-theme
pub const LIGHTER: Theme = Theme {
    name: Cow::Borrowed("Material Lighter"),
    author: Cow::Borrowed("Mattia Astorino"),
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfafafa),
    fg: Color::from_hex(0x546e7a),
    cursor: Some(Color::from_hex(0x546e7a)),
    selection: Some(Color::from_hex(0xe7e7e8)),
    line_highlight: Some(Color::from_hex(0xe7e7e8)),
    gutter: Some(Color::from_hex(0xccd7da)),
    statusbar_bg: Some(Color::from_hex(0xeceff1)),
    statusbar_fg: Some(Color::from_hex(0x90a4ae)),
    comment: Some(Color::from_hex(0xaabfc9)),
    keyword: Some(Color::from_hex(0x7c4dff)),
    string: Some(Color::from_hex(0x91b859)),
    function: Some(Color::from_hex(0x6182b8)),
    variable: Some(Color::from_hex(0xf07178)),
    r#type: Some(Color::from_hex(0xffb62c)),
    constant: Some(Color::from_hex(0xf76d47)),
    operator: Some(Color::from_hex(0x39adb5)),
    tag: Some(Color::from_hex(0xe53935)),
    error: Some(Color::from_hex(0xe53935)),
    warning: Some(Color::from_hex(0xffb62c)),
    info: Some(Color::from_hex(0x6182b8)),
    success: Some(Color::from_hex(0x91b859)),
    red: Some(Color::from_hex(0xe53935)),
    orange: Some(Color::from_hex(0xf76d47)),
    yellow: Some(Color::from_hex(0xffb62c)),
    green: Some(Color::from_hex(0x91b859)),
    cyan: Some(Color::from_hex(0x39adb5)),
    blue: Some(Color::from_hex(0x6182b8)),
    purple: Some(Color::from_hex(0x7c4dff)),
    magenta: Some(Color::from_hex(0xe53935)),
};
