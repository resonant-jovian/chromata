//! SynthWave '84 color theme.
//!
//! A retro-futuristic neon-tinted theme by Robb Owen.
//! <https://github.com/robb0wen/synthwave-vscode>

use crate::{Color, Contrast, Theme, Variant};

/// SynthWave '84 — neon dark theme.
///
/// Author: Robb Owen
/// Variant: Dark
/// Contrast: Normal
/// Source: synthwave-vscode
pub const THEME: Theme = Theme {
    name: "SynthWave '84",
    author: "Robb Owen",
    variant: Variant::Dark,
    contrast: Contrast::Normal,
    bg: Color::from_hex(0x262335),
    fg: Color::from_hex(0xbbbbbb),
    cursor: Some(Color::from_hex(0xbbbbbb)),
    selection: Some(Color::from_hex(0x34294f)),
    line_highlight: Some(Color::from_hex(0x2a2139)),
    gutter: Some(Color::from_hex(0x495495)),
    statusbar_bg: Some(Color::from_hex(0x2a2139)),
    statusbar_fg: Some(Color::from_hex(0x848bbd)),
    comment: Some(Color::from_hex(0x848bbd)),
    keyword: Some(Color::from_hex(0xfede5d)),
    string: Some(Color::from_hex(0xff8b39)),
    function: Some(Color::from_hex(0x36f9f6)),
    variable: Some(Color::from_hex(0xff7edb)),
    r#type: Some(Color::from_hex(0x72f1b8)),
    constant: Some(Color::from_hex(0xf97e72)),
    operator: Some(Color::from_hex(0xfede5d)),
    tag: Some(Color::from_hex(0xfe4450)),
    error: Some(Color::from_hex(0xfe4450)),
    warning: Some(Color::from_hex(0xfede5d)),
    info: Some(Color::from_hex(0x36f9f6)),
    success: Some(Color::from_hex(0x72f1b8)),
    red: Some(Color::from_hex(0xfe4450)),
    orange: Some(Color::from_hex(0xff8b39)),
    yellow: Some(Color::from_hex(0xfede5d)),
    green: Some(Color::from_hex(0x72f1b8)),
    cyan: Some(Color::from_hex(0x36f9f6)),
    blue: Some(Color::from_hex(0x03edf9)),
    purple: Some(Color::from_hex(0xff7edb)),
    magenta: Some(Color::from_hex(0xfe4450)),
};
