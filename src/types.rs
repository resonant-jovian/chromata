use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

/// A color represented as RGB components.
/// All values are compile-time constants with zero runtime cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Color {
    /// Red component (0–255).
    pub r: u8,
    /// Green component (0–255).
    pub g: u8,
    /// Blue component (0–255).
    pub b: u8,
}

impl Color {
    /// Construct a Color from RGB components.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let red = Color::new(255, 0, 0);
    /// assert_eq!(red.r, 255);
    /// assert_eq!(red.g, 0);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Construct a Color from a 24-bit hex value (0xRRGGBB).
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let c = Color::from_hex(0x1d2021);
    /// assert_eq!(c.r, 0x1d);
    /// assert_eq!(c.g, 0x20);
    /// assert_eq!(c.b, 0x21);
    /// ```
    pub const fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    /// Return the color as a 24-bit hex value.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let c = Color::new(0x1d, 0x20, 0x21);
    /// assert_eq!(c.to_hex(), 0x1d2021);
    /// ```
    pub const fn to_hex(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Return the color as a CSS hex string like "#1d2021".
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let c = Color::from_hex(0x1d2021);
    /// assert_eq!(c.to_css_hex(), "#1d2021");
    /// ```
    pub fn to_css_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Convert to an (f32, f32, f32) tuple in `[0.0, 1.0]` range.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let white = Color::new(255, 255, 255);
    /// let (r, g, b) = white.to_f32();
    /// assert!((r - 1.0).abs() < f32::EPSILON);
    /// ```
    pub const fn to_f32(self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    /// Relative luminance per WCAG 2.0.
    ///
    /// Returns a value between 0.0 (black) and 1.0 (white).
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let black = Color::new(0, 0, 0);
    /// let white = Color::new(255, 255, 255);
    /// assert!((black.luminance()).abs() < 0.001);
    /// assert!((white.luminance() - 1.0).abs() < 0.001);
    /// ```
    pub fn luminance(self) -> f64 {
        let r = srgb_to_linear(self.r as f64 / 255.0);
        let g = srgb_to_linear(self.g as f64 / 255.0);
        let b = srgb_to_linear(self.b as f64 / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// WCAG contrast ratio between two colors.
    ///
    /// Returns a value between 1.0 (identical) and 21.0 (black vs white).
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let black = Color::new(0, 0, 0);
    /// let white = Color::new(255, 255, 255);
    /// let ratio = black.contrast_ratio(white);
    /// assert!(ratio > 20.0); // ~21:1 for black/white
    /// ```
    pub fn contrast_ratio(self, other: Color) -> f64 {
        let l1 = self.luminance();
        let l2 = other.luminance();
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Linear interpolation between two colors.
    ///
    /// `t` is clamped to `[0.0, 1.0]`. Interpolation is performed in sRGB space.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let black = Color::new(0, 0, 0);
    /// let white = Color::new(255, 255, 255);
    /// let mid = black.lerp(white, 0.5);
    /// assert_eq!(mid, Color::new(127, 127, 127));
    /// ```
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
        }
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
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Variant {
    /// A dark theme (light text on dark background).
    Dark,
    /// A light theme (dark text on light background).
    Light,
}

/// Formats the variant as "Dark" or "Light".
///
/// # Examples
///
/// ```
/// use chromata::Variant;
///
/// assert_eq!(format!("{}", Variant::Dark), "Dark");
/// assert_eq!(format!("{}", Variant::Light), "Light");
/// ```
impl core::fmt::Display for Variant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Variant::Dark => f.write_str("Dark"),
            Variant::Light => f.write_str("Light"),
        }
    }
}

/// Contrast level classification based on WCAG contrast ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Contrast {
    /// High contrast (WCAG ratio >= 10.0).
    High,
    /// Normal contrast (WCAG ratio 4.5–10.0).
    Normal,
    /// Low contrast (WCAG ratio < 4.5).
    Low,
}

/// Formats the contrast level as "High", "Normal", or "Low".
///
/// # Examples
///
/// ```
/// use chromata::Contrast;
///
/// assert_eq!(format!("{}", Contrast::High), "High");
/// assert_eq!(format!("{}", Contrast::Normal), "Normal");
/// ```
impl core::fmt::Display for Contrast {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Contrast::High => f.write_str("High"),
            Contrast::Normal => f.write_str("Normal"),
            Contrast::Low => f.write_str("Low"),
        }
    }
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// A complete editor/terminal color theme.
///
/// All optional fields are `Option<Color>` because not every source theme
/// defines every semantic role. The `bg`, `fg`, and at least some syntax
/// colors will always be `Some` for any theme that passed validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
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
    /// Comment text color.
    pub comment: Option<Color>,
    /// Language keyword color (if, else, fn, struct, ...).
    pub keyword: Option<Color>,
    /// String literal color.
    pub string: Option<Color>,
    /// Function/method name color.
    pub function: Option<Color>,
    /// Variable/parameter name color.
    pub variable: Option<Color>,
    /// Type name color (structs, enums, traits, ...).
    pub r#type: Option<Color>,
    /// Constant/literal value color.
    pub constant: Option<Color>,
    /// Operator color (+, -, =, ...).
    pub operator: Option<Color>,
    /// Tag color (HTML/XML tags).
    pub tag: Option<Color>,

    // -- Diagnostic colors --
    /// Error diagnostic color.
    pub error: Option<Color>,
    /// Warning diagnostic color.
    pub warning: Option<Color>,
    /// Info diagnostic color.
    pub info: Option<Color>,
    /// Success/hint diagnostic color.
    pub success: Option<Color>,

    // -- Named accent palette --
    /// Red accent.
    pub red: Option<Color>,
    /// Orange accent.
    pub orange: Option<Color>,
    /// Yellow accent.
    pub yellow: Option<Color>,
    /// Green accent.
    pub green: Option<Color>,
    /// Cyan accent.
    pub cyan: Option<Color>,
    /// Blue accent.
    pub blue: Option<Color>,
    /// Purple accent.
    pub purple: Option<Color>,
    /// Magenta/pink accent.
    pub magenta: Option<Color>,
}

impl Theme {
    /// Is this a dark theme?
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = &chromata::popular::gruvbox::DARK_HARD;
    /// assert!(theme.is_dark());
    /// ```
    pub const fn is_dark(&self) -> bool {
        matches!(self.variant, Variant::Dark)
    }

    /// Return the first available accent color in priority order.
    ///
    /// Checks: blue, purple, cyan, green, orange, red. Falls back to `fg`.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = &chromata::popular::gruvbox::DARK_HARD;
    /// let accent = theme.accent();
    /// // Gruvbox has blue defined, so accent returns blue
    /// assert_eq!(accent, theme.blue.unwrap());
    /// ```
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
    ///
    /// Always includes at least `("bg", ...)` and `("fg", ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = &chromata::popular::gruvbox::DARK_HARD;
    /// let colors = theme.colors();
    /// assert!(colors.len() >= 2);
    /// assert_eq!(colors[0].0, "bg");
    /// assert_eq!(colors[1].0, "fg");
    /// ```
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
        push_opt!(r#type);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
pub struct Base16Palette {
    /// Default background.
    pub base00: Color,
    /// Lighter background (status bars, line highlights).
    pub base01: Color,
    /// Selection background.
    pub base02: Color,
    /// Comments, invisibles, line highlighting.
    pub base03: Color,
    /// Dark foreground (status bars).
    pub base04: Color,
    /// Default foreground, caret, delimiters.
    pub base05: Color,
    /// Light foreground.
    pub base06: Color,
    /// Lightest foreground.
    pub base07: Color,
    /// Variables, XML tags, markup link text, diff deleted.
    pub base08: Color,
    /// Integers, boolean, constants, XML attributes.
    pub base09: Color,
    /// Classes, markup bold, search text background.
    pub base0a: Color,
    /// Strings, inherited class, diff inserted.
    pub base0b: Color,
    /// Support, regex, escape characters.
    pub base0c: Color,
    /// Functions, methods, headings.
    pub base0d: Color,
    /// Keywords, storage, selector.
    pub base0e: Color,
    /// Deprecated, opening/closing embedded language tags.
    pub base0f: Color,
}

/// Extended base24 theme with additional accent slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
pub struct Base24Palette {
    /// Base16 palette (slots 00–0F).
    pub base: Base16Palette,
    /// Darker background.
    pub base10: Color,
    /// Darkest background.
    pub base11: Color,
    /// Bright red.
    pub base12: Color,
    /// Bright yellow.
    pub base13: Color,
    /// Bright green.
    pub base14: Color,
    /// Bright cyan.
    pub base15: Color,
    /// Bright blue.
    pub base16: Color,
    /// Bright magenta.
    pub base17: Color,
}
