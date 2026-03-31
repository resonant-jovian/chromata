//! Tomorrow color themes.
//!
//! A bright, pastel-toned color scheme by Chris Kempson.
//! <https://github.com/chriskempson/tomorrow-theme>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Tomorrow Night — dark variant.
///
/// Author: Chris Kempson
/// Variant: Dark
/// Contrast: Normal
/// Source: tomorrow-theme
pub const NIGHT: Theme = Theme {
    name: Cow::Borrowed("Tomorrow Night"),
    author: Cow::Borrowed("Chris Kempson"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x1d1f21),
    fg: Color::from_hex(0xc5c8c6),
    cursor: Some(Color::from_hex(0xc5c8c6)),
    selection: Some(Color::from_hex(0x373b41)),
    line_highlight: Some(Color::from_hex(0x282a2e)),
    gutter: Some(Color::from_hex(0x969896)),
    statusbar_bg: Some(Color::from_hex(0x282a2e)),
    statusbar_fg: Some(Color::from_hex(0xb4b7b4)),
    comment: Some(Color::from_hex(0x969896)),
    keyword: Some(Color::from_hex(0xb294bb)),
    string: Some(Color::from_hex(0xb5bd68)),
    function: Some(Color::from_hex(0x81a2be)),
    variable: Some(Color::from_hex(0xcc6666)),
    r#type: Some(Color::from_hex(0xf0c674)),
    constant: Some(Color::from_hex(0xde935f)),
    operator: Some(Color::from_hex(0x8abeb7)),
    tag: Some(Color::from_hex(0xcc6666)),
    error: Some(Color::from_hex(0xcc6666)),
    warning: Some(Color::from_hex(0xf0c674)),
    info: Some(Color::from_hex(0x81a2be)),
    success: Some(Color::from_hex(0xb5bd68)),
    red: Some(Color::from_hex(0xcc6666)),
    orange: Some(Color::from_hex(0xde935f)),
    yellow: Some(Color::from_hex(0xf0c674)),
    green: Some(Color::from_hex(0xb5bd68)),
    cyan: Some(Color::from_hex(0x8abeb7)),
    blue: Some(Color::from_hex(0x81a2be)),
    purple: Some(Color::from_hex(0xb294bb)),
    magenta: Some(Color::from_hex(0xcc6666)),
};

/// Tomorrow Night Eighties — muted dark variant.
///
/// Author: Chris Kempson
/// Variant: Dark
/// Contrast: Normal
/// Source: tomorrow-theme
pub const NIGHT_EIGHTIES: Theme = Theme {
    name: Cow::Borrowed("Tomorrow Night Eighties"),
    author: Cow::Borrowed("Chris Kempson"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x2d2d2d),
    fg: Color::from_hex(0xcccccc),
    cursor: Some(Color::from_hex(0xcccccc)),
    selection: Some(Color::from_hex(0x515151)),
    line_highlight: Some(Color::from_hex(0x393939)),
    gutter: Some(Color::from_hex(0x999999)),
    statusbar_bg: Some(Color::from_hex(0x393939)),
    statusbar_fg: Some(Color::from_hex(0xb4b7b4)),
    comment: Some(Color::from_hex(0x999999)),
    keyword: Some(Color::from_hex(0xcc99cc)),
    string: Some(Color::from_hex(0x99cc99)),
    function: Some(Color::from_hex(0x6699cc)),
    variable: Some(Color::from_hex(0xf2777a)),
    r#type: Some(Color::from_hex(0xffcc66)),
    constant: Some(Color::from_hex(0xf99157)),
    operator: Some(Color::from_hex(0x66cccc)),
    tag: Some(Color::from_hex(0xf2777a)),
    error: Some(Color::from_hex(0xf2777a)),
    warning: Some(Color::from_hex(0xffcc66)),
    info: Some(Color::from_hex(0x6699cc)),
    success: Some(Color::from_hex(0x99cc99)),
    red: Some(Color::from_hex(0xf2777a)),
    orange: Some(Color::from_hex(0xf99157)),
    yellow: Some(Color::from_hex(0xffcc66)),
    green: Some(Color::from_hex(0x99cc99)),
    cyan: Some(Color::from_hex(0x66cccc)),
    blue: Some(Color::from_hex(0x6699cc)),
    purple: Some(Color::from_hex(0xcc99cc)),
    magenta: Some(Color::from_hex(0xf2777a)),
};

/// Tomorrow Light — light variant.
///
/// Author: Chris Kempson
/// Variant: Light
/// Contrast: Normal
/// Source: tomorrow-theme
pub const LIGHT: Theme = Theme {
    name: Cow::Borrowed("Tomorrow Light"),
    author: Cow::Borrowed("Chris Kempson"),
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xffffff),
    fg: Color::from_hex(0x4d4d4c),
    cursor: Some(Color::from_hex(0x4d4d4c)),
    selection: Some(Color::from_hex(0xd6d6d6)),
    line_highlight: Some(Color::from_hex(0xefefef)),
    gutter: Some(Color::from_hex(0x8e908c)),
    statusbar_bg: Some(Color::from_hex(0xefefef)),
    statusbar_fg: Some(Color::from_hex(0x4d4d4c)),
    comment: Some(Color::from_hex(0x8e908c)),
    keyword: Some(Color::from_hex(0x8959a8)),
    string: Some(Color::from_hex(0x718c00)),
    function: Some(Color::from_hex(0x4271ae)),
    variable: Some(Color::from_hex(0xc82829)),
    r#type: Some(Color::from_hex(0xeab700)),
    constant: Some(Color::from_hex(0xf5871f)),
    operator: Some(Color::from_hex(0x3e999f)),
    tag: Some(Color::from_hex(0xc82829)),
    error: Some(Color::from_hex(0xc82829)),
    warning: Some(Color::from_hex(0xeab700)),
    info: Some(Color::from_hex(0x4271ae)),
    success: Some(Color::from_hex(0x718c00)),
    red: Some(Color::from_hex(0xc82829)),
    orange: Some(Color::from_hex(0xf5871f)),
    yellow: Some(Color::from_hex(0xeab700)),
    green: Some(Color::from_hex(0x718c00)),
    cyan: Some(Color::from_hex(0x3e999f)),
    blue: Some(Color::from_hex(0x4271ae)),
    purple: Some(Color::from_hex(0x8959a8)),
    magenta: Some(Color::from_hex(0xc82829)),
};
