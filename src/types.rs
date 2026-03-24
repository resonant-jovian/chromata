use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

/// A color represented as RGB components.
/// All values are compile-time constants with zero runtime cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Construct a Color from RGB components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Construct a Color from a 24-bit hex value (0xRRGGBB).
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Return the color as a 24-bit hex value.
    pub const fn to_hex(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Return the color as a CSS hex string like "#1d2021".
    pub fn to_css_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Convert to an (f32, f32, f32) tuple in [0.0, 1.0] range.
    pub const fn to_f32(self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    /// Relative luminance per WCAG 2.0.
    pub fn luminance(self) -> f64 {
        let r = srgb_to_linear(self.r as f64 / 255.0);
        let g = srgb_to_linear(self.g as f64 / 255.0);
        let b = srgb_to_linear(self.b as f64 / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// WCAG contrast ratio between two colors.
    pub fn contrast_ratio(self, other: Color) -> f64 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }
}

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        libm::pow((c + 0.055) / 1.055, 2.4)
    }
}

/// Dark or light theme variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Variant {
    Dark,
    Light,
}

/// Contrast level classification based on WCAG contrast ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Contrast {
    High,
    Normal,
    Low,
}

/// A complete editor/terminal color theme.
///
/// All optional fields are `Option<Color>` because not every source theme
/// defines every semantic role. The `bg`, `fg`, and at least some syntax
/// colors will always be `Some` for any theme that passed validation.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    // -- Metadata --
    /// Human-readable theme name, e.g. "Gruvbox Dark Hard"
    pub name: &'static str,
    /// Theme author
    pub author: &'static str,
    /// `Variant::Dark` or `Variant::Light`
    pub variant: Variant,
    /// `Contrast::High`, `Contrast::Normal`, or `Contrast::Low`
    pub contrast: Contrast,

    // -- Core UI colors --
    /// Editor/terminal background
    pub bg: Color,
    /// Default foreground text
    pub fg: Color,
    /// Cursor color
    pub cursor: Option<Color>,
    /// Visual selection / highlight background
    pub selection: Option<Color>,
    /// Current line highlight background
    pub line_highlight: Option<Color>,
    /// Line number gutter foreground
    pub gutter: Option<Color>,
    /// Status bar / mode line background
    pub statusbar_bg: Option<Color>,
    /// Status bar / mode line foreground
    pub statusbar_fg: Option<Color>,

    // -- Syntax colors --
    pub comment: Option<Color>,
    pub keyword: Option<Color>,
    pub string: Option<Color>,
    pub function: Option<Color>,
    pub variable: Option<Color>,
    pub r#type: Option<Color>,
    pub constant: Option<Color>,
    pub operator: Option<Color>,
    pub tag: Option<Color>,

    // -- Diagnostic colors --
    pub error: Option<Color>,
    pub warning: Option<Color>,
    pub info: Option<Color>,
    pub success: Option<Color>,

    // -- Named accent palette --
    pub red: Option<Color>,
    pub orange: Option<Color>,
    pub yellow: Option<Color>,
    pub green: Option<Color>,
    pub cyan: Option<Color>,
    pub blue: Option<Color>,
    pub purple: Option<Color>,
    pub magenta: Option<Color>,
}

impl Theme {
    /// Is this a dark theme?
    pub const fn is_dark(&self) -> bool {
        matches!(self.variant, Variant::Dark)
    }

    /// Return the first available accent color in priority order.
    /// Useful for generic "accent" when you don't care which hue.
    pub const fn accent(&self) -> Color {
        if let Some(c) = self.blue {
            return c;
        }
        if let Some(c) = self.purple {
            return c;
        }
        if let Some(c) = self.cyan {
            return c;
        }
        if let Some(c) = self.green {
            return c;
        }
        if let Some(c) = self.orange {
            return c;
        }
        if let Some(c) = self.red {
            return c;
        }
        self.fg
    }

    /// All defined colors as a vec of (role_name, Color) pairs.
    pub fn colors(&self) -> Vec<(&'static str, Color)> {
        let mut out = vec![("bg", self.bg), ("fg", self.fg)];
        macro_rules! push_opt {
            ($field:ident) => {
                if let Some(c) = self.$field {
                    out.push((stringify!($field), c));
                }
            };
        }
        push_opt!(cursor);
        push_opt!(selection);
        push_opt!(line_highlight);
        push_opt!(gutter);
        push_opt!(statusbar_bg);
        push_opt!(statusbar_fg);
        push_opt!(comment);
        push_opt!(keyword);
        push_opt!(string);
        push_opt!(function);
        push_opt!(variable);
        push_opt!(constant);
        push_opt!(operator);
        push_opt!(tag);
        push_opt!(error);
        push_opt!(warning);
        push_opt!(info);
        push_opt!(success);
        push_opt!(red);
        push_opt!(orange);
        push_opt!(yellow);
        push_opt!(green);
        push_opt!(cyan);
        push_opt!(blue);
        push_opt!(purple);
        push_opt!(magenta);
        out
    }
}

/// The 16 base16 palette slots.
#[derive(Debug, Clone, Copy)]
pub struct Base16Palette {
    pub base00: Color,
    pub base01: Color,
    pub base02: Color,
    pub base03: Color,
    pub base04: Color,
    pub base05: Color,
    pub base06: Color,
    pub base07: Color,
    pub base08: Color,
    pub base09: Color,
    pub base0a: Color,
    pub base0b: Color,
    pub base0c: Color,
    pub base0d: Color,
    pub base0e: Color,
    pub base0f: Color,
}

/// Extended base24 theme with additional accent slots.
#[derive(Debug, Clone, Copy)]
pub struct Base24Palette {
    pub base: Base16Palette,
    pub base10: Color,
    pub base11: Color,
    pub base12: Color,
    pub base13: Color,
    pub base14: Color,
    pub base15: Color,
    pub base16: Color,
    pub base17: Color,
}
