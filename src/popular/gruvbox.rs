//! Gruvbox color themes.
//!
//! A retro groove color scheme by Dawid Kurek (morhetz).
//! <https://github.com/morhetz/gruvbox>

use crate::{Color, Contrast, Theme, Variant};

/// Gruvbox Dark Hard — high-contrast dark variant.
///
/// Author: Dawid Kurek (dawikur@gmail.com)
/// Variant: Dark
/// Contrast: High
/// Source: base16 (tinted-theming/schemes)
pub const DARK_HARD: Theme = Theme {
    name: "Gruvbox Dark Hard",
    author: "Dawid Kurek (dawikur@gmail.com)",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x1d2021),
    fg: Color::from_hex(0xd5c4a1),
    cursor: Some(Color::from_hex(0xd5c4a1)),
    selection: Some(Color::from_hex(0x504945)),
    line_highlight: Some(Color::from_hex(0x3c3836)),
    gutter: Some(Color::from_hex(0x665c54)),
    statusbar_bg: Some(Color::from_hex(0x3c3836)),
    statusbar_fg: Some(Color::from_hex(0xbdae93)),
    comment: Some(Color::from_hex(0x665c54)),
    keyword: Some(Color::from_hex(0xd3869b)),
    string: Some(Color::from_hex(0xb8bb26)),
    function: Some(Color::from_hex(0x83a598)),
    variable: Some(Color::from_hex(0xfb4934)),
    r#type: Some(Color::from_hex(0xfabd2f)),
    constant: Some(Color::from_hex(0xfe8019)),
    operator: Some(Color::from_hex(0xd5c4a1)),
    tag: Some(Color::from_hex(0xfb4934)),
    error: Some(Color::from_hex(0xfb4934)),
    warning: Some(Color::from_hex(0xfabd2f)),
    info: Some(Color::from_hex(0x83a598)),
    success: Some(Color::from_hex(0xb8bb26)),
    red: Some(Color::from_hex(0xfb4934)),
    orange: Some(Color::from_hex(0xfe8019)),
    yellow: Some(Color::from_hex(0xfabd2f)),
    green: Some(Color::from_hex(0xb8bb26)),
    cyan: Some(Color::from_hex(0x8ec07c)),
    blue: Some(Color::from_hex(0x83a598)),
    purple: Some(Color::from_hex(0xd3869b)),
    magenta: Some(Color::from_hex(0xd65d0e)),
};

/// Gruvbox Dark — standard dark variant.
///
/// Author: Dawid Kurek (dawikur@gmail.com)
/// Variant: Dark
/// Contrast: Normal
/// Source: base16 (tinted-theming/schemes)
pub const DARK: Theme = Theme {
    name: "Gruvbox Dark",
    author: "Dawid Kurek (dawikur@gmail.com)",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x282828),
    fg: Color::from_hex(0xd5c4a1),
    cursor: Some(Color::from_hex(0xd5c4a1)),
    selection: Some(Color::from_hex(0x504945)),
    line_highlight: Some(Color::from_hex(0x3c3836)),
    gutter: Some(Color::from_hex(0x665c54)),
    statusbar_bg: Some(Color::from_hex(0x3c3836)),
    statusbar_fg: Some(Color::from_hex(0xbdae93)),
    comment: Some(Color::from_hex(0x665c54)),
    keyword: Some(Color::from_hex(0xd3869b)),
    string: Some(Color::from_hex(0xb8bb26)),
    function: Some(Color::from_hex(0x83a598)),
    variable: Some(Color::from_hex(0xfb4934)),
    r#type: Some(Color::from_hex(0xfabd2f)),
    constant: Some(Color::from_hex(0xfe8019)),
    operator: Some(Color::from_hex(0xd5c4a1)),
    tag: Some(Color::from_hex(0xfb4934)),
    error: Some(Color::from_hex(0xfb4934)),
    warning: Some(Color::from_hex(0xfabd2f)),
    info: Some(Color::from_hex(0x83a598)),
    success: Some(Color::from_hex(0xb8bb26)),
    red: Some(Color::from_hex(0xfb4934)),
    orange: Some(Color::from_hex(0xfe8019)),
    yellow: Some(Color::from_hex(0xfabd2f)),
    green: Some(Color::from_hex(0xb8bb26)),
    cyan: Some(Color::from_hex(0x8ec07c)),
    blue: Some(Color::from_hex(0x83a598)),
    purple: Some(Color::from_hex(0xd3869b)),
    magenta: Some(Color::from_hex(0xd65d0e)),
};

/// Gruvbox Light — light variant.
///
/// Author: Dawid Kurek (dawikur@gmail.com)
/// Variant: Light
/// Contrast: Normal
/// Source: base16 (tinted-theming/schemes)
pub const LIGHT: Theme = Theme {
    name: "Gruvbox Light",
    author: "Dawid Kurek (dawikur@gmail.com)",
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfbf1c7),
    fg: Color::from_hex(0x504945),
    cursor: Some(Color::from_hex(0x504945)),
    selection: Some(Color::from_hex(0xd5c4a1)),
    line_highlight: Some(Color::from_hex(0xebdbb2)),
    gutter: Some(Color::from_hex(0xbdae93)),
    statusbar_bg: Some(Color::from_hex(0xebdbb2)),
    statusbar_fg: Some(Color::from_hex(0x665c54)),
    comment: Some(Color::from_hex(0xbdae93)),
    keyword: Some(Color::from_hex(0xd3869b)),
    string: Some(Color::from_hex(0xb8bb26)),
    function: Some(Color::from_hex(0x83a598)),
    variable: Some(Color::from_hex(0xfb4934)),
    r#type: Some(Color::from_hex(0xfabd2f)),
    constant: Some(Color::from_hex(0xfe8019)),
    operator: Some(Color::from_hex(0x504945)),
    tag: Some(Color::from_hex(0xfb4934)),
    error: Some(Color::from_hex(0xfb4934)),
    warning: Some(Color::from_hex(0xfabd2f)),
    info: Some(Color::from_hex(0x83a598)),
    success: Some(Color::from_hex(0xb8bb26)),
    red: Some(Color::from_hex(0xfb4934)),
    orange: Some(Color::from_hex(0xfe8019)),
    yellow: Some(Color::from_hex(0xfabd2f)),
    green: Some(Color::from_hex(0xb8bb26)),
    cyan: Some(Color::from_hex(0x8ec07c)),
    blue: Some(Color::from_hex(0x83a598)),
    purple: Some(Color::from_hex(0xd3869b)),
    magenta: Some(Color::from_hex(0xd65d0e)),
};
