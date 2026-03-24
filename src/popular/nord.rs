//! Nord color theme.
//!
//! An arctic, north-bluish color palette by Arctic Ice Studio.
//! <https://github.com/nordtheme/nord>

use crate::{Color, Contrast, Theme, Variant};

/// Nord — arctic dark theme.
///
/// Author: Arctic Ice Studio
/// Variant: Dark
/// Contrast: Normal
/// Source: <https://www.nordtheme.com/>
pub const THEME: Theme = Theme {
    name: "Nord",
    author: "Arctic Ice Studio",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x2e3440),
    fg: Color::from_hex(0xd8dee9),
    cursor: Some(Color::from_hex(0xd8dee9)),
    selection: Some(Color::from_hex(0x434c5e)),
    line_highlight: Some(Color::from_hex(0x3b4252)),
    gutter: Some(Color::from_hex(0x4c566a)),
    statusbar_bg: Some(Color::from_hex(0x3b4252)),
    statusbar_fg: Some(Color::from_hex(0xe5e9f0)),
    comment: Some(Color::from_hex(0x4c566a)),
    keyword: Some(Color::from_hex(0x81a1c1)),
    string: Some(Color::from_hex(0xa3be8c)),
    function: Some(Color::from_hex(0x88c0d0)),
    variable: Some(Color::from_hex(0xd08770)),
    r#type: Some(Color::from_hex(0xebcb8b)),
    constant: Some(Color::from_hex(0xb48ead)),
    operator: Some(Color::from_hex(0x81a1c1)),
    tag: Some(Color::from_hex(0xbf616a)),
    error: Some(Color::from_hex(0xbf616a)),
    warning: Some(Color::from_hex(0xebcb8b)),
    info: Some(Color::from_hex(0x88c0d0)),
    success: Some(Color::from_hex(0xa3be8c)),
    red: Some(Color::from_hex(0xbf616a)),
    orange: Some(Color::from_hex(0xd08770)),
    yellow: Some(Color::from_hex(0xebcb8b)),
    green: Some(Color::from_hex(0xa3be8c)),
    cyan: Some(Color::from_hex(0x8fbcbb)),
    blue: Some(Color::from_hex(0x81a1c1)),
    purple: Some(Color::from_hex(0xb48ead)),
    magenta: Some(Color::from_hex(0xbf616a)),
};
