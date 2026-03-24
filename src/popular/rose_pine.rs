//! Rosé Pine color themes.
//!
//! All natural pine, faux fur and a bit of soho vibes for the classy minimalist.
//! <https://github.com/rose-pine/rose-pine-theme>

use crate::{Color, Contrast, Theme, Variant};

/// Rosé Pine — the original dark variant.
///
/// Author: Rosé Pine
/// Variant: Dark
/// Contrast: High
/// Source: rose-pine-theme
pub const THEME: Theme = Theme {
    name: "Rosé Pine",
    author: "Rosé Pine",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x191724),
    fg: Color::from_hex(0xe0def4),
    cursor: Some(Color::from_hex(0xe0def4)),
    selection: Some(Color::from_hex(0x2a283e)),
    line_highlight: Some(Color::from_hex(0x21202e)),
    gutter: Some(Color::from_hex(0x6e6a86)),
    statusbar_bg: Some(Color::from_hex(0x1f1d2e)),
    statusbar_fg: Some(Color::from_hex(0x908caa)),
    comment: Some(Color::from_hex(0x6e6a86)),
    keyword: Some(Color::from_hex(0xc4a7e7)),
    string: Some(Color::from_hex(0xf6c177)),
    function: Some(Color::from_hex(0xeb6f92)),
    variable: Some(Color::from_hex(0x9ccfd8)),
    r#type: Some(Color::from_hex(0xebbcba)),
    constant: Some(Color::from_hex(0xc4a7e7)),
    operator: Some(Color::from_hex(0x31748f)),
    tag: Some(Color::from_hex(0xeb6f92)),
    error: Some(Color::from_hex(0xeb6f92)),
    warning: Some(Color::from_hex(0xf6c177)),
    info: Some(Color::from_hex(0x9ccfd8)),
    success: Some(Color::from_hex(0xf6c177)),
    red: Some(Color::from_hex(0xeb6f92)),
    orange: Some(Color::from_hex(0xebbcba)),
    yellow: Some(Color::from_hex(0xf6c177)),
    green: Some(Color::from_hex(0x31748f)),
    cyan: Some(Color::from_hex(0x9ccfd8)),
    blue: Some(Color::from_hex(0x31748f)),
    purple: Some(Color::from_hex(0xc4a7e7)),
    magenta: Some(Color::from_hex(0xeb6f92)),
};

/// Rosé Pine Moon — darker moon variant.
///
/// Author: Rosé Pine
/// Variant: Dark
/// Contrast: High
/// Source: rose-pine-theme
pub const MOON: Theme = Theme {
    name: "Rosé Pine Moon",
    author: "Rosé Pine",
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x232136),
    fg: Color::from_hex(0xe0def4),
    cursor: Some(Color::from_hex(0xe0def4)),
    selection: Some(Color::from_hex(0x393552)),
    line_highlight: Some(Color::from_hex(0x2a273f)),
    gutter: Some(Color::from_hex(0x6e6a86)),
    statusbar_bg: Some(Color::from_hex(0x2a273f)),
    statusbar_fg: Some(Color::from_hex(0x908caa)),
    comment: Some(Color::from_hex(0x6e6a86)),
    keyword: Some(Color::from_hex(0xc4a7e7)),
    string: Some(Color::from_hex(0xf6c177)),
    function: Some(Color::from_hex(0xeb6f92)),
    variable: Some(Color::from_hex(0x9ccfd8)),
    r#type: Some(Color::from_hex(0xea9a97)),
    constant: Some(Color::from_hex(0xc4a7e7)),
    operator: Some(Color::from_hex(0x3e8fb0)),
    tag: Some(Color::from_hex(0xeb6f92)),
    error: Some(Color::from_hex(0xeb6f92)),
    warning: Some(Color::from_hex(0xf6c177)),
    info: Some(Color::from_hex(0x9ccfd8)),
    success: Some(Color::from_hex(0xf6c177)),
    red: Some(Color::from_hex(0xeb6f92)),
    orange: Some(Color::from_hex(0xea9a97)),
    yellow: Some(Color::from_hex(0xf6c177)),
    green: Some(Color::from_hex(0x3e8fb0)),
    cyan: Some(Color::from_hex(0x9ccfd8)),
    blue: Some(Color::from_hex(0x3e8fb0)),
    purple: Some(Color::from_hex(0xc4a7e7)),
    magenta: Some(Color::from_hex(0xeb6f92)),
};

/// Rosé Pine Dawn — light variant.
///
/// Author: Rosé Pine
/// Variant: Light
/// Contrast: Normal
/// Source: rose-pine-theme
pub const DAWN: Theme = Theme {
    name: "Rosé Pine Dawn",
    author: "Rosé Pine",
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xfaf4ed),
    fg: Color::from_hex(0x575279),
    cursor: Some(Color::from_hex(0x575279)),
    selection: Some(Color::from_hex(0xdfdad9)),
    line_highlight: Some(Color::from_hex(0xf2e9e1)),
    gutter: Some(Color::from_hex(0x9893a5)),
    statusbar_bg: Some(Color::from_hex(0xf2e9e1)),
    statusbar_fg: Some(Color::from_hex(0x797593)),
    comment: Some(Color::from_hex(0x9893a5)),
    keyword: Some(Color::from_hex(0x907aa9)),
    string: Some(Color::from_hex(0xea9d34)),
    function: Some(Color::from_hex(0xb4637a)),
    variable: Some(Color::from_hex(0x56949f)),
    r#type: Some(Color::from_hex(0xd7827e)),
    constant: Some(Color::from_hex(0x907aa9)),
    operator: Some(Color::from_hex(0x286983)),
    tag: Some(Color::from_hex(0xb4637a)),
    error: Some(Color::from_hex(0xb4637a)),
    warning: Some(Color::from_hex(0xea9d34)),
    info: Some(Color::from_hex(0x56949f)),
    success: Some(Color::from_hex(0xea9d34)),
    red: Some(Color::from_hex(0xb4637a)),
    orange: Some(Color::from_hex(0xd7827e)),
    yellow: Some(Color::from_hex(0xea9d34)),
    green: Some(Color::from_hex(0x286983)),
    cyan: Some(Color::from_hex(0x56949f)),
    blue: Some(Color::from_hex(0x286983)),
    purple: Some(Color::from_hex(0x907aa9)),
    magenta: Some(Color::from_hex(0xb4637a)),
};
