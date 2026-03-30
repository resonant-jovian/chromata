//! GitHub color themes.
//!
//! The official GitHub editor color schemes.
//! <https://github.com/primer/github-vscode-theme>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// GitHub Dark — dark variant.
///
/// Author: GitHub (Primer)
/// Variant: Dark
/// Contrast: High
/// Source: github-vscode-theme
pub const DARK: Theme = Theme {
    name: Cow::Borrowed("GitHub Dark"),
    author: Cow::Borrowed("GitHub (Primer)"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x0d1117),
    fg: Color::from_hex(0xc9d1d9),
    cursor: Some(Color::from_hex(0xc9d1d9)),
    selection: Some(Color::from_hex(0x1c2128)),
    line_highlight: Some(Color::from_hex(0x161b22)),
    gutter: Some(Color::from_hex(0x484f58)),
    statusbar_bg: Some(Color::from_hex(0x161b22)),
    statusbar_fg: Some(Color::from_hex(0x8b949e)),
    comment: Some(Color::from_hex(0x8b949e)),
    keyword: Some(Color::from_hex(0xff7b72)),
    string: Some(Color::from_hex(0xa5d6ff)),
    function: Some(Color::from_hex(0xd2a8ff)),
    variable: Some(Color::from_hex(0xffa657)),
    r#type: Some(Color::from_hex(0x79c0ff)),
    constant: Some(Color::from_hex(0x79c0ff)),
    operator: Some(Color::from_hex(0xff7b72)),
    tag: Some(Color::from_hex(0x7ee787)),
    error: Some(Color::from_hex(0xf85149)),
    warning: Some(Color::from_hex(0xd29922)),
    info: Some(Color::from_hex(0x58a6ff)),
    success: Some(Color::from_hex(0x3fb950)),
    red: Some(Color::from_hex(0xf85149)),
    orange: Some(Color::from_hex(0xd29922)),
    yellow: Some(Color::from_hex(0xe3b341)),
    green: Some(Color::from_hex(0x3fb950)),
    cyan: Some(Color::from_hex(0x39d353)),
    blue: Some(Color::from_hex(0x58a6ff)),
    purple: Some(Color::from_hex(0xd2a8ff)),
    magenta: Some(Color::from_hex(0xf778ba)),
};

/// GitHub Light — light variant.
///
/// Author: GitHub (Primer)
/// Variant: Light
/// Contrast: High
/// Source: github-vscode-theme
pub const LIGHT: Theme = Theme {
    name: Cow::Borrowed("GitHub Light"),
    author: Cow::Borrowed("GitHub (Primer)"),
    variant: Variant::Light,
    contrast: Contrast::High,
    bg: Color::from_hex(0xffffff),
    fg: Color::from_hex(0x1f2328),
    cursor: Some(Color::from_hex(0x1f2328)),
    selection: Some(Color::from_hex(0xddf4ff)),
    line_highlight: Some(Color::from_hex(0xf6f8fa)),
    gutter: Some(Color::from_hex(0x8c959f)),
    statusbar_bg: Some(Color::from_hex(0xf6f8fa)),
    statusbar_fg: Some(Color::from_hex(0x59636e)),
    comment: Some(Color::from_hex(0x59636e)),
    keyword: Some(Color::from_hex(0xcf222e)),
    string: Some(Color::from_hex(0x0a3069)),
    function: Some(Color::from_hex(0x8250df)),
    variable: Some(Color::from_hex(0x953800)),
    r#type: Some(Color::from_hex(0x0550ae)),
    constant: Some(Color::from_hex(0x0550ae)),
    operator: Some(Color::from_hex(0xcf222e)),
    tag: Some(Color::from_hex(0x116329)),
    error: Some(Color::from_hex(0xcf222e)),
    warning: Some(Color::from_hex(0x9a6700)),
    info: Some(Color::from_hex(0x0550ae)),
    success: Some(Color::from_hex(0x116329)),
    red: Some(Color::from_hex(0xcf222e)),
    orange: Some(Color::from_hex(0xbc4c00)),
    yellow: Some(Color::from_hex(0x9a6700)),
    green: Some(Color::from_hex(0x116329)),
    cyan: Some(Color::from_hex(0x1b7c83)),
    blue: Some(Color::from_hex(0x0550ae)),
    purple: Some(Color::from_hex(0x8250df)),
    magenta: Some(Color::from_hex(0xbf3989)),
};
