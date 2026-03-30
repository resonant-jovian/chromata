//! Catppuccin color themes.
//!
//! A soothing pastel theme for the high-spirited by the catppuccin org.
//! <https://github.com/catppuccin/catppuccin>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Catppuccin Latte — light variant.
///
/// Author: catppuccin org
/// Variant: Light
/// Contrast: Normal
/// Source: <https://catppuccin.com/>
pub const LATTE: Theme = Theme {
    name: Cow::Borrowed("Catppuccin Latte"),
    author: Cow::Borrowed("catppuccin org"),
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xeff1f5),
    fg: Color::from_hex(0x4c4f69),
    cursor: Some(Color::from_hex(0x4c4f69)),
    selection: Some(Color::from_hex(0xccd0da)),
    line_highlight: Some(Color::from_hex(0xe6e9ef)),
    gutter: Some(Color::from_hex(0x9ca0b0)),
    statusbar_bg: Some(Color::from_hex(0xe6e9ef)),
    statusbar_fg: Some(Color::from_hex(0x5c5f77)),
    comment: Some(Color::from_hex(0x9ca0b0)),
    keyword: Some(Color::from_hex(0x8839ef)),
    string: Some(Color::from_hex(0x40a02b)),
    function: Some(Color::from_hex(0x1e66f5)),
    variable: Some(Color::from_hex(0xdf8e1d)),
    r#type: Some(Color::from_hex(0xfe640b)),
    constant: Some(Color::from_hex(0xea76cb)),
    operator: Some(Color::from_hex(0x04a5e5)),
    tag: Some(Color::from_hex(0xd20f39)),
    error: Some(Color::from_hex(0xd20f39)),
    warning: Some(Color::from_hex(0xdf8e1d)),
    info: Some(Color::from_hex(0x1e66f5)),
    success: Some(Color::from_hex(0x40a02b)),
    red: Some(Color::from_hex(0xd20f39)),
    orange: Some(Color::from_hex(0xfe640b)),
    yellow: Some(Color::from_hex(0xdf8e1d)),
    green: Some(Color::from_hex(0x40a02b)),
    cyan: Some(Color::from_hex(0x04a5e5)),
    blue: Some(Color::from_hex(0x1e66f5)),
    purple: Some(Color::from_hex(0x8839ef)),
    magenta: Some(Color::from_hex(0xea76cb)),
};

/// Catppuccin Frappe — dark variant.
///
/// Author: catppuccin org
/// Variant: Dark
/// Contrast: Normal
/// Source: <https://catppuccin.com/>
pub const FRAPPE: Theme = Theme {
    name: Cow::Borrowed("Catppuccin Frappe"),
    author: Cow::Borrowed("catppuccin org"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x303446),
    fg: Color::from_hex(0xc6d0f5),
    cursor: Some(Color::from_hex(0xc6d0f5)),
    selection: Some(Color::from_hex(0x414559)),
    line_highlight: Some(Color::from_hex(0x292c3c)),
    gutter: Some(Color::from_hex(0x737994)),
    statusbar_bg: Some(Color::from_hex(0x292c3c)),
    statusbar_fg: Some(Color::from_hex(0xb5bfe2)),
    comment: Some(Color::from_hex(0x737994)),
    keyword: Some(Color::from_hex(0xca9ee6)),
    string: Some(Color::from_hex(0xa6d189)),
    function: Some(Color::from_hex(0x8caaee)),
    variable: Some(Color::from_hex(0xe5c890)),
    r#type: Some(Color::from_hex(0xef9f76)),
    constant: Some(Color::from_hex(0xf4b8e4)),
    operator: Some(Color::from_hex(0x99d1db)),
    tag: Some(Color::from_hex(0xe78284)),
    error: Some(Color::from_hex(0xe78284)),
    warning: Some(Color::from_hex(0xe5c890)),
    info: Some(Color::from_hex(0x8caaee)),
    success: Some(Color::from_hex(0xa6d189)),
    red: Some(Color::from_hex(0xe78284)),
    orange: Some(Color::from_hex(0xef9f76)),
    yellow: Some(Color::from_hex(0xe5c890)),
    green: Some(Color::from_hex(0xa6d189)),
    cyan: Some(Color::from_hex(0x99d1db)),
    blue: Some(Color::from_hex(0x8caaee)),
    purple: Some(Color::from_hex(0xca9ee6)),
    magenta: Some(Color::from_hex(0xf4b8e4)),
};

/// Catppuccin Macchiato — dark variant.
///
/// Author: catppuccin org
/// Variant: Dark
/// Contrast: Normal
/// Source: <https://catppuccin.com/>
pub const MACCHIATO: Theme = Theme {
    name: Cow::Borrowed("Catppuccin Macchiato"),
    author: Cow::Borrowed("catppuccin org"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x24273a),
    fg: Color::from_hex(0xcad3f5),
    cursor: Some(Color::from_hex(0xcad3f5)),
    selection: Some(Color::from_hex(0x363a4f)),
    line_highlight: Some(Color::from_hex(0x1e2030)),
    gutter: Some(Color::from_hex(0x6e738d)),
    statusbar_bg: Some(Color::from_hex(0x1e2030)),
    statusbar_fg: Some(Color::from_hex(0xb8c0e0)),
    comment: Some(Color::from_hex(0x6e738d)),
    keyword: Some(Color::from_hex(0xc6a0f6)),
    string: Some(Color::from_hex(0xa6da95)),
    function: Some(Color::from_hex(0x8aadf4)),
    variable: Some(Color::from_hex(0xeed49f)),
    r#type: Some(Color::from_hex(0xf5a97f)),
    constant: Some(Color::from_hex(0xf5bde6)),
    operator: Some(Color::from_hex(0x91d7e3)),
    tag: Some(Color::from_hex(0xed8796)),
    error: Some(Color::from_hex(0xed8796)),
    warning: Some(Color::from_hex(0xeed49f)),
    info: Some(Color::from_hex(0x8aadf4)),
    success: Some(Color::from_hex(0xa6da95)),
    red: Some(Color::from_hex(0xed8796)),
    orange: Some(Color::from_hex(0xf5a97f)),
    yellow: Some(Color::from_hex(0xeed49f)),
    green: Some(Color::from_hex(0xa6da95)),
    cyan: Some(Color::from_hex(0x91d7e3)),
    blue: Some(Color::from_hex(0x8aadf4)),
    purple: Some(Color::from_hex(0xc6a0f6)),
    magenta: Some(Color::from_hex(0xf5bde6)),
};

/// Catppuccin Mocha — dark variant.
///
/// Author: catppuccin org
/// Variant: Dark
/// Contrast: High
/// Source: <https://catppuccin.com/>
pub const MOCHA: Theme = Theme {
    name: Cow::Borrowed("Catppuccin Mocha"),
    author: Cow::Borrowed("catppuccin org"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x1e1e2e),
    fg: Color::from_hex(0xcdd6f4),
    cursor: Some(Color::from_hex(0xcdd6f4)),
    selection: Some(Color::from_hex(0x313244)),
    line_highlight: Some(Color::from_hex(0x181825)),
    gutter: Some(Color::from_hex(0x6c7086)),
    statusbar_bg: Some(Color::from_hex(0x181825)),
    statusbar_fg: Some(Color::from_hex(0xbac2de)),
    comment: Some(Color::from_hex(0x6c7086)),
    keyword: Some(Color::from_hex(0xcba6f7)),
    string: Some(Color::from_hex(0xa6e3a1)),
    function: Some(Color::from_hex(0x89b4fa)),
    variable: Some(Color::from_hex(0xf9e2af)),
    r#type: Some(Color::from_hex(0xfab387)),
    constant: Some(Color::from_hex(0xf5c2e7)),
    operator: Some(Color::from_hex(0x89dceb)),
    tag: Some(Color::from_hex(0xf38ba8)),
    error: Some(Color::from_hex(0xf38ba8)),
    warning: Some(Color::from_hex(0xf9e2af)),
    info: Some(Color::from_hex(0x89b4fa)),
    success: Some(Color::from_hex(0xa6e3a1)),
    red: Some(Color::from_hex(0xf38ba8)),
    orange: Some(Color::from_hex(0xfab387)),
    yellow: Some(Color::from_hex(0xf9e2af)),
    green: Some(Color::from_hex(0xa6e3a1)),
    cyan: Some(Color::from_hex(0x89dceb)),
    blue: Some(Color::from_hex(0x89b4fa)),
    purple: Some(Color::from_hex(0xcba6f7)),
    magenta: Some(Color::from_hex(0xf5c2e7)),
};
