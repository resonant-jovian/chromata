//! Tokyo Night color themes.
//!
//! A clean dark theme that celebrates the lights of downtown Tokyo at night.
//! <https://github.com/enkia/tokyo-night-vscode-theme>

use crate::{Color, Contrast, Theme, Variant};
use alloc::borrow::Cow;

/// Tokyo Night Dark — the original dark variant.
///
/// Author: Enkia
/// Variant: Dark
/// Contrast: Normal
/// Source: tokyo-night-vscode-theme
pub const DARK: Theme = Theme {
    name: Cow::Borrowed("Tokyo Night Dark"),
    author: Cow::Borrowed("Enkia"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x1a1b26),
    fg: Color::from_hex(0xa9b1d6),
    cursor: Some(Color::from_hex(0xc0caf5)),
    selection: Some(Color::from_hex(0x283457)),
    line_highlight: Some(Color::from_hex(0x292e42)),
    gutter: Some(Color::from_hex(0x3b4261)),
    statusbar_bg: Some(Color::from_hex(0x16161e)),
    statusbar_fg: Some(Color::from_hex(0x737aa2)),
    comment: Some(Color::from_hex(0x565f89)),
    keyword: Some(Color::from_hex(0x9d7cd8)),
    string: Some(Color::from_hex(0x9ece6a)),
    function: Some(Color::from_hex(0x7aa2f7)),
    variable: Some(Color::from_hex(0xe0af68)),
    r#type: Some(Color::from_hex(0xff9e64)),
    constant: Some(Color::from_hex(0xbb9af7)),
    operator: Some(Color::from_hex(0x89ddff)),
    tag: Some(Color::from_hex(0xf7768e)),
    error: Some(Color::from_hex(0xf7768e)),
    warning: Some(Color::from_hex(0xe0af68)),
    info: Some(Color::from_hex(0x7aa2f7)),
    success: Some(Color::from_hex(0x9ece6a)),
    red: Some(Color::from_hex(0xf7768e)),
    orange: Some(Color::from_hex(0xff9e64)),
    yellow: Some(Color::from_hex(0xe0af68)),
    green: Some(Color::from_hex(0x9ece6a)),
    cyan: Some(Color::from_hex(0x7dcfff)),
    blue: Some(Color::from_hex(0x7aa2f7)),
    purple: Some(Color::from_hex(0x9d7cd8)),
    magenta: Some(Color::from_hex(0xbb9af7)),
};

/// Tokyo Night Storm — darker background variant.
///
/// Author: Enkia
/// Variant: Dark
/// Contrast: Normal
/// Source: tokyo-night-vscode-theme
pub const STORM: Theme = Theme {
    name: Cow::Borrowed("Tokyo Night Storm"),
    author: Cow::Borrowed("Enkia"),
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x24283b),
    fg: Color::from_hex(0xa9b1d6),
    cursor: Some(Color::from_hex(0xc0caf5)),
    selection: Some(Color::from_hex(0x283457)),
    line_highlight: Some(Color::from_hex(0x292e42)),
    gutter: Some(Color::from_hex(0x3b4261)),
    statusbar_bg: Some(Color::from_hex(0x1f2335)),
    statusbar_fg: Some(Color::from_hex(0x737aa2)),
    comment: Some(Color::from_hex(0x565f89)),
    keyword: Some(Color::from_hex(0x9d7cd8)),
    string: Some(Color::from_hex(0x9ece6a)),
    function: Some(Color::from_hex(0x7aa2f7)),
    variable: Some(Color::from_hex(0xe0af68)),
    r#type: Some(Color::from_hex(0xff9e64)),
    constant: Some(Color::from_hex(0xbb9af7)),
    operator: Some(Color::from_hex(0x89ddff)),
    tag: Some(Color::from_hex(0xf7768e)),
    error: Some(Color::from_hex(0xf7768e)),
    warning: Some(Color::from_hex(0xe0af68)),
    info: Some(Color::from_hex(0x7aa2f7)),
    success: Some(Color::from_hex(0x9ece6a)),
    red: Some(Color::from_hex(0xf7768e)),
    orange: Some(Color::from_hex(0xff9e64)),
    yellow: Some(Color::from_hex(0xe0af68)),
    green: Some(Color::from_hex(0x9ece6a)),
    cyan: Some(Color::from_hex(0x7dcfff)),
    blue: Some(Color::from_hex(0x7aa2f7)),
    purple: Some(Color::from_hex(0x9d7cd8)),
    magenta: Some(Color::from_hex(0xbb9af7)),
};

/// Tokyo Night Light — light variant (Day).
///
/// Author: Enkia
/// Variant: Light
/// Contrast: Normal
/// Source: tokyo-night-vscode-theme
pub const LIGHT: Theme = Theme {
    name: Cow::Borrowed("Tokyo Night Light"),
    author: Cow::Borrowed("Enkia"),
    variant: Variant::Light,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0xd5d6db),
    fg: Color::from_hex(0x343b58),
    cursor: Some(Color::from_hex(0x343b58)),
    selection: Some(Color::from_hex(0xb6b8c3)),
    line_highlight: Some(Color::from_hex(0xc4c5cb)),
    gutter: Some(Color::from_hex(0x9699a3)),
    statusbar_bg: Some(Color::from_hex(0xc4c5cb)),
    statusbar_fg: Some(Color::from_hex(0x6172b0)),
    comment: Some(Color::from_hex(0x9699a3)),
    keyword: Some(Color::from_hex(0x5a4a78)),
    string: Some(Color::from_hex(0x485e30)),
    function: Some(Color::from_hex(0x34548a)),
    variable: Some(Color::from_hex(0x8f5e15)),
    r#type: Some(Color::from_hex(0x965027)),
    constant: Some(Color::from_hex(0x7847bd)),
    operator: Some(Color::from_hex(0x166775)),
    tag: Some(Color::from_hex(0x8c4351)),
    error: Some(Color::from_hex(0x8c4351)),
    warning: Some(Color::from_hex(0x8f5e15)),
    info: Some(Color::from_hex(0x34548a)),
    success: Some(Color::from_hex(0x485e30)),
    red: Some(Color::from_hex(0x8c4351)),
    orange: Some(Color::from_hex(0x965027)),
    yellow: Some(Color::from_hex(0x8f5e15)),
    green: Some(Color::from_hex(0x485e30)),
    cyan: Some(Color::from_hex(0x166775)),
    blue: Some(Color::from_hex(0x34548a)),
    purple: Some(Color::from_hex(0x5a4a78)),
    magenta: Some(Color::from_hex(0x7847bd)),
};
