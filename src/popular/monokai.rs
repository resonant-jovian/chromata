//! Monokai color themes.
//!
//! A classic dark theme by Wimer Hazenberg.
//! <https://monokai.pro/>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Monokai Classic — the original Monokai theme.
///
/// Author: Wimer Hazenberg
/// Variant: Dark
/// Contrast: High
/// Source: <https://monokai.pro/>
pub const THEME: Theme = Theme {
    name: Cow::Borrowed("Monokai Classic"),
    author: Cow::Borrowed("Wimer Hazenberg"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x272822),
    fg: Color::from_hex(0xf8f8f2),
    cursor: Some(Color::from_hex(0xf8f8f2)),
    selection: Some(Color::from_hex(0x49483e)),
    line_highlight: Some(Color::from_hex(0x3e3d32)),
    gutter: Some(Color::from_hex(0x75715e)),
    statusbar_bg: Some(Color::from_hex(0x3e3d32)),
    statusbar_fg: Some(Color::from_hex(0xa59f85)),
    comment: Some(Color::from_hex(0x75715e)),
    keyword: Some(Color::from_hex(0xf92672)),
    string: Some(Color::from_hex(0xe6db74)),
    function: Some(Color::from_hex(0xa6e22e)),
    variable: Some(Color::from_hex(0xfd971f)),
    r#type: Some(Color::from_hex(0x66d9ef)),
    constant: Some(Color::from_hex(0xae81ff)),
    operator: Some(Color::from_hex(0xf92672)),
    tag: Some(Color::from_hex(0xf92672)),
    error: Some(Color::from_hex(0xf92672)),
    warning: Some(Color::from_hex(0xe6db74)),
    info: Some(Color::from_hex(0x66d9ef)),
    success: Some(Color::from_hex(0xa6e22e)),
    red: Some(Color::from_hex(0xf92672)),
    orange: Some(Color::from_hex(0xfd971f)),
    yellow: Some(Color::from_hex(0xe6db74)),
    green: Some(Color::from_hex(0xa6e22e)),
    cyan: Some(Color::from_hex(0x66d9ef)),
    blue: Some(Color::from_hex(0x66d9ef)),
    purple: Some(Color::from_hex(0xae81ff)),
    magenta: Some(Color::from_hex(0xf92672)),
};

/// Monokai Pro — the modern evolution of Monokai.
///
/// Author: Wimer Hazenberg
/// Variant: Dark
/// Contrast: High
/// Source: <https://monokai.pro/>
pub const PRO: Theme = Theme {
    name: Cow::Borrowed("Monokai Pro"),
    author: Cow::Borrowed("Wimer Hazenberg"),
    variant: Variant::Dark,
    contrast: Contrast::High,
    bg: Color::from_hex(0x2d2a2e),
    fg: Color::from_hex(0xfcfcfa),
    cursor: Some(Color::from_hex(0xfcfcfa)),
    selection: Some(Color::from_hex(0x403e41)),
    line_highlight: Some(Color::from_hex(0x363437)),
    gutter: Some(Color::from_hex(0x727072)),
    statusbar_bg: Some(Color::from_hex(0x363437)),
    statusbar_fg: Some(Color::from_hex(0x939293)),
    comment: Some(Color::from_hex(0x727072)),
    keyword: Some(Color::from_hex(0xff6188)),
    string: Some(Color::from_hex(0xffd866)),
    function: Some(Color::from_hex(0xa9dc76)),
    variable: Some(Color::from_hex(0xfc9867)),
    r#type: Some(Color::from_hex(0x78dce8)),
    constant: Some(Color::from_hex(0xab9df2)),
    operator: Some(Color::from_hex(0xff6188)),
    tag: Some(Color::from_hex(0xff6188)),
    error: Some(Color::from_hex(0xff6188)),
    warning: Some(Color::from_hex(0xffd866)),
    info: Some(Color::from_hex(0x78dce8)),
    success: Some(Color::from_hex(0xa9dc76)),
    red: Some(Color::from_hex(0xff6188)),
    orange: Some(Color::from_hex(0xfc9867)),
    yellow: Some(Color::from_hex(0xffd866)),
    green: Some(Color::from_hex(0xa9dc76)),
    cyan: Some(Color::from_hex(0x78dce8)),
    blue: Some(Color::from_hex(0x78dce8)),
    purple: Some(Color::from_hex(0xab9df2)),
    magenta: Some(Color::from_hex(0xff6188)),
};
