use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

/// Error returned when parsing a [`Color`] from a string fails.
///
/// Returned by `"#xyz".parse::<Color>()` when the input is not a valid
/// 3-digit or 6-digit CSS hex color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError;

impl core::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid CSS hex color (expected #RGB, RGB, #RRGGBB, or RRGGBB)")
    }
}

impl core::error::Error for ParseColorError {}

/// Error returned when parsing a [`Variant`] from a string fails.
///
/// Returned by `"xyz".parse::<Variant>()` when the input is not
/// "dark" or "light" (case-insensitive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseVariantError;

impl core::fmt::Display for ParseVariantError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid variant (expected \"Dark\" or \"Light\")")
    }
}

impl core::error::Error for ParseVariantError {}

/// Error returned when parsing a [`Contrast`] from a string fails.
///
/// Returned by `"xyz".parse::<Contrast>()` when the input is not
/// "high", "normal", or "low" (case-insensitive).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseContrastError;

impl core::fmt::Display for ParseContrastError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid contrast (expected \"High\", \"Normal\", or \"Low\")")
    }
}

impl core::error::Error for ParseContrastError {}

const fn hex_digit(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// A color represented as RGB components.
/// All values are compile-time constants with zero runtime cost.
///
/// Ordering is lexicographic by `(r, g, b)`.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    /// Construct a Color by parsing a CSS hex string.
    ///
    /// Accepts 6-digit (`"#1d2021"`, `"1d2021"`) and 3-digit shorthand
    /// (`"#FFF"`, `"FFF"`) formats. The 3-digit form expands each digit
    /// (e.g., `#ABC` becomes `#AABBCC`).
    ///
    /// Returns `None` if the string is not a valid hex color.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let c = Color::from_css_hex("#1d2021").unwrap();
    /// assert_eq!(c, Color::from_hex(0x1d2021));
    ///
    /// let c = Color::from_css_hex("1d2021").unwrap();
    /// assert_eq!(c, Color::from_hex(0x1d2021));
    ///
    /// let c = Color::from_css_hex("#FFF").unwrap();
    /// assert_eq!(c, Color::new(255, 255, 255));
    ///
    /// assert!(Color::from_css_hex("nope").is_none());
    /// ```
    #[must_use]
    pub const fn from_css_hex(s: &str) -> Option<Color> {
        let bytes = s.as_bytes();

        // Strip leading '#' and determine length
        let (has_hash, hex_len) = if !bytes.is_empty() && bytes[0] == b'#' {
            (true, bytes.len() - 1)
        } else {
            (false, bytes.len())
        };

        let start = if has_hash { 1 } else { 0 };

        match hex_len {
            6 => {
                let (r_hi, r_lo) = (hex_digit(bytes[start]), hex_digit(bytes[start + 1]));
                let (g_hi, g_lo) = (hex_digit(bytes[start + 2]), hex_digit(bytes[start + 3]));
                let (b_hi, b_lo) = (hex_digit(bytes[start + 4]), hex_digit(bytes[start + 5]));

                match (r_hi, r_lo, g_hi, g_lo, b_hi, b_lo) {
                    (Some(rh), Some(rl), Some(gh), Some(gl), Some(bh), Some(bl)) => Some(Color {
                        r: rh << 4 | rl,
                        g: gh << 4 | gl,
                        b: bh << 4 | bl,
                    }),
                    _ => None,
                }
            }
            3 => {
                let (r, g, b) = (
                    hex_digit(bytes[start]),
                    hex_digit(bytes[start + 1]),
                    hex_digit(bytes[start + 2]),
                );

                match (r, g, b) {
                    (Some(r), Some(g), Some(b)) => Some(Color {
                        r: r << 4 | r,
                        g: g << 4 | g,
                        b: b << 4 | b,
                    }),
                    _ => None,
                }
            }
            _ => None,
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub const fn to_f32(self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    /// Construct a Color from normalized `[0.0, 1.0]` RGB components.
    ///
    /// Values are clamped to `[0.0, 1.0]` and rounded to the nearest `u8`.
    /// `NaN` is treated as `0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Color;
    ///
    /// let c = Color::from_f32(1.0, 0.5, 0.0);
    /// assert_eq!(c, Color::new(255, 128, 0));
    /// ```
    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0 + 0.5) as u8,
        }
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
    #[must_use]
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
    #[must_use]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Variant {
    /// A dark theme (light text on dark background).
    #[default]
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

/// Parses "Dark" or "Light" (case-insensitive).
///
/// # Examples
///
/// ```
/// use chromata::Variant;
///
/// let v: Variant = "dark".parse().unwrap();
/// assert_eq!(v, Variant::Dark);
/// ```
impl core::str::FromStr for Variant {
    type Err = ParseVariantError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("dark") {
            Ok(Variant::Dark)
        } else if s.eq_ignore_ascii_case("light") {
            Ok(Variant::Light)
        } else {
            Err(ParseVariantError)
        }
    }
}

/// Contrast level classification based on WCAG contrast ratio.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Contrast {
    /// High contrast (WCAG ratio >= 10.0).
    High,
    /// Normal contrast (WCAG ratio 4.5–10.0).
    #[default]
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

impl Contrast {
    /// Classify a WCAG contrast ratio into a contrast level.
    ///
    /// - `High` if ratio >= 10.0
    /// - `Normal` if ratio >= 4.5
    /// - `Low` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::Contrast;
    ///
    /// assert_eq!(Contrast::from_ratio(12.0), Contrast::High);
    /// assert_eq!(Contrast::from_ratio(7.0), Contrast::Normal);
    /// assert_eq!(Contrast::from_ratio(2.0), Contrast::Low);
    /// ```
    #[must_use]
    pub const fn from_ratio(ratio: f64) -> Self {
        if ratio >= 10.0 {
            Contrast::High
        } else if ratio >= 4.5 {
            Contrast::Normal
        } else {
            Contrast::Low
        }
    }
}

/// Parses "High", "Normal", or "Low" (case-insensitive).
///
/// # Examples
///
/// ```
/// use chromata::Contrast;
///
/// let c: Contrast = "high".parse().unwrap();
/// assert_eq!(c, Contrast::High);
/// ```
impl core::str::FromStr for Contrast {
    type Err = ParseContrastError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("high") {
            Ok(Contrast::High)
        } else if s.eq_ignore_ascii_case("normal") {
            Ok(Contrast::Normal)
        } else if s.eq_ignore_ascii_case("low") {
            Ok(Contrast::Low)
        } else {
            Err(ParseContrastError)
        }
    }
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Construct a Color from a 24-bit hex value (0xRRGGBB).
///
/// Only the lower 24 bits are used; upper bits are silently ignored.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let c: Color = 0x1d2021u32.into();
/// assert_eq!(c, Color::from_hex(0x1d2021));
/// ```
impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Self::from_hex(hex)
    }
}

/// Construct a Color from an (r, g, b) tuple.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let c: Color = (29, 32, 33).into();
/// assert_eq!(c, Color::new(29, 32, 33));
/// ```
impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}

/// Extract the 24-bit hex value from a Color.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let v: u32 = Color::from_hex(0x1d2021).into();
/// assert_eq!(v, 0x1d2021);
/// ```
impl From<Color> for u32 {
    fn from(c: Color) -> Self {
        c.to_hex()
    }
}

/// Extract the (r, g, b) components from a Color.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let (r, g, b): (u8, u8, u8) = Color::new(29, 32, 33).into();
/// assert_eq!((r, g, b), (29, 32, 33));
/// ```
impl From<Color> for (u8, u8, u8) {
    fn from(c: Color) -> Self {
        (c.r, c.g, c.b)
    }
}

/// Construct a Color from an `[r, g, b]` array.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let c: Color = [29, 32, 33].into();
/// assert_eq!(c, Color::new(29, 32, 33));
/// ```
impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::new(r, g, b)
    }
}

/// Extract the `[r, g, b]` components from a Color.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let arr: [u8; 3] = Color::new(29, 32, 33).into();
/// assert_eq!(arr, [29, 32, 33]);
/// ```
impl From<Color> for [u8; 3] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b]
    }
}

/// Default Color is black (0, 0, 0).
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// assert_eq!(Color::default(), Color::new(0, 0, 0));
/// ```
impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

/// Parse a CSS hex color string like `"#1d2021"`, `"1d2021"`, `"#FFF"`, or `"FFF"`.
///
/// # Examples
///
/// ```
/// use chromata::Color;
///
/// let c: Color = "#1d2021".parse().unwrap();
/// assert_eq!(c, Color::from_hex(0x1d2021));
/// ```
impl core::str::FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_css_hex(s).ok_or(ParseColorError)
    }
}

/// A complete editor/terminal color theme.
///
/// Fields `bg` and `fg` are always present. Other color fields are
/// `Option<Color>` because not every source theme defines every semantic role.
///
/// With `serde-support`, themes can be serialized and deserialized (e.g., JSON).
/// Compile-time theme constants use `Cow::Borrowed` for zero-cost access;
/// deserialized themes use `Cow::Owned`.
///
/// This struct is `#[non_exhaustive]` — use [`Theme::builder`] to construct
/// themes outside the crate. Future fields must be `Option<T>` to maintain
/// serde deserialization compatibility with earlier versions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[non_exhaustive]
pub struct Theme {
    // -- Metadata --
    /// Human-readable theme name, e.g. "Gruvbox Dark Hard"
    pub name: Cow<'static, str>,
    /// Theme author
    pub author: Cow<'static, str>,
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
    #[must_use]
    pub const fn is_dark(&self) -> bool {
        matches!(self.variant, Variant::Dark)
    }

    /// Is this a light theme?
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = &chromata::popular::gruvbox::LIGHT;
    /// assert!(theme.is_light());
    /// ```
    #[must_use]
    pub const fn is_light(&self) -> bool {
        matches!(self.variant, Variant::Light)
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

    /// Create a [`ThemeBuilder`] for constructing a theme at runtime.
    ///
    /// Accepts `&'static str` or `String` for name/author. Variant and contrast
    /// are auto-detected from `bg`/`fg` if not set explicitly.
    ///
    /// # Examples
    ///
    /// ```
    /// use chromata::{Color, Theme, Variant};
    ///
    /// let theme = Theme::builder("My Theme", "Me", Color::new(0, 0, 0), Color::new(255, 255, 255))
    ///     .keyword(Color::from_hex(0xff79c6))
    ///     .build();
    /// assert_eq!(theme.name, "My Theme");
    /// assert!(theme.is_dark());
    /// ```
    pub fn builder(
        name: impl Into<Cow<'static, str>>,
        author: impl Into<Cow<'static, str>>,
        bg: Color,
        fg: Color,
    ) -> ThemeBuilder {
        ThemeBuilder::new(name, author, bg, fg)
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
    #[must_use]
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

/// Builder for constructing [`Theme`] values at runtime.
///
/// Required fields (name, author, bg, fg) are set in [`ThemeBuilder::new`].
/// All other fields default to `None`. Variant is auto-detected from `bg`
/// luminance and contrast is auto-calculated from the `bg`/`fg` ratio
/// unless set explicitly.
///
/// # Examples
///
/// ```
/// use chromata::{Color, Theme, ThemeBuilder};
///
/// let theme = ThemeBuilder::new("My Theme", "Author", Color::new(0, 0, 0), Color::new(255, 255, 255))
///     .keyword(Color::from_hex(0xff79c6))
///     .string(Color::from_hex(0xf1fa8c))
///     .build();
/// assert!(theme.is_dark());
/// ```
#[must_use = "builders do nothing until .build() is called"]
#[derive(Debug, Clone)]
pub struct ThemeBuilder {
    name: Cow<'static, str>,
    author: Cow<'static, str>,
    bg: Color,
    fg: Color,
    variant: Option<Variant>,
    contrast: Option<Contrast>,
    cursor: Option<Color>,
    selection: Option<Color>,
    line_highlight: Option<Color>,
    gutter: Option<Color>,
    statusbar_bg: Option<Color>,
    statusbar_fg: Option<Color>,
    comment: Option<Color>,
    keyword: Option<Color>,
    string: Option<Color>,
    function: Option<Color>,
    variable: Option<Color>,
    r#type: Option<Color>,
    constant: Option<Color>,
    operator: Option<Color>,
    tag: Option<Color>,
    error: Option<Color>,
    warning: Option<Color>,
    info: Option<Color>,
    success: Option<Color>,
    red: Option<Color>,
    orange: Option<Color>,
    yellow: Option<Color>,
    green: Option<Color>,
    cyan: Option<Color>,
    blue: Option<Color>,
    purple: Option<Color>,
    magenta: Option<Color>,
}

impl ThemeBuilder {
    /// Create a new builder with the required fields.
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        author: impl Into<Cow<'static, str>>,
        bg: Color,
        fg: Color,
    ) -> Self {
        Self {
            name: name.into(),
            author: author.into(),
            bg,
            fg,
            variant: None,
            contrast: None,
            cursor: None,
            selection: None,
            line_highlight: None,
            gutter: None,
            statusbar_bg: None,
            statusbar_fg: None,
            comment: None,
            keyword: None,
            string: None,
            function: None,
            variable: None,
            r#type: None,
            constant: None,
            operator: None,
            tag: None,
            error: None,
            warning: None,
            info: None,
            success: None,
            red: None,
            orange: None,
            yellow: None,
            green: None,
            cyan: None,
            blue: None,
            purple: None,
            magenta: None,
        }
    }

    /// Override the auto-detected variant.
    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = Some(v);
        self
    }

    /// Override the auto-calculated contrast.
    pub fn contrast(mut self, c: Contrast) -> Self {
        self.contrast = Some(c);
        self
    }

    /// Set the cursor color.
    pub fn cursor(mut self, c: Color) -> Self {
        self.cursor = Some(c);
        self
    }

    /// Set the selection color.
    pub fn selection(mut self, c: Color) -> Self {
        self.selection = Some(c);
        self
    }

    /// Set the line highlight color.
    pub fn line_highlight(mut self, c: Color) -> Self {
        self.line_highlight = Some(c);
        self
    }

    /// Set the gutter color.
    pub fn gutter(mut self, c: Color) -> Self {
        self.gutter = Some(c);
        self
    }

    /// Set the status bar background color.
    pub fn statusbar_bg(mut self, c: Color) -> Self {
        self.statusbar_bg = Some(c);
        self
    }

    /// Set the status bar foreground color.
    pub fn statusbar_fg(mut self, c: Color) -> Self {
        self.statusbar_fg = Some(c);
        self
    }

    /// Set the comment color.
    pub fn comment(mut self, c: Color) -> Self {
        self.comment = Some(c);
        self
    }

    /// Set the keyword color.
    pub fn keyword(mut self, c: Color) -> Self {
        self.keyword = Some(c);
        self
    }

    /// Set the string color.
    pub fn string(mut self, c: Color) -> Self {
        self.string = Some(c);
        self
    }

    /// Set the function color.
    pub fn function(mut self, c: Color) -> Self {
        self.function = Some(c);
        self
    }

    /// Set the variable color.
    pub fn variable(mut self, c: Color) -> Self {
        self.variable = Some(c);
        self
    }

    /// Set the type color.
    pub fn r#type(mut self, c: Color) -> Self {
        self.r#type = Some(c);
        self
    }

    /// Set the constant color.
    pub fn constant(mut self, c: Color) -> Self {
        self.constant = Some(c);
        self
    }

    /// Set the operator color.
    pub fn operator(mut self, c: Color) -> Self {
        self.operator = Some(c);
        self
    }

    /// Set the tag color.
    pub fn tag(mut self, c: Color) -> Self {
        self.tag = Some(c);
        self
    }

    /// Set the error color.
    pub fn error(mut self, c: Color) -> Self {
        self.error = Some(c);
        self
    }

    /// Set the warning color.
    pub fn warning(mut self, c: Color) -> Self {
        self.warning = Some(c);
        self
    }

    /// Set the info color.
    pub fn info(mut self, c: Color) -> Self {
        self.info = Some(c);
        self
    }

    /// Set the success color.
    pub fn success(mut self, c: Color) -> Self {
        self.success = Some(c);
        self
    }

    /// Set the red accent color.
    pub fn red(mut self, c: Color) -> Self {
        self.red = Some(c);
        self
    }

    /// Set the orange accent color.
    pub fn orange(mut self, c: Color) -> Self {
        self.orange = Some(c);
        self
    }

    /// Set the yellow accent color.
    pub fn yellow(mut self, c: Color) -> Self {
        self.yellow = Some(c);
        self
    }

    /// Set the green accent color.
    pub fn green(mut self, c: Color) -> Self {
        self.green = Some(c);
        self
    }

    /// Set the cyan accent color.
    pub fn cyan(mut self, c: Color) -> Self {
        self.cyan = Some(c);
        self
    }

    /// Set the blue accent color.
    pub fn blue(mut self, c: Color) -> Self {
        self.blue = Some(c);
        self
    }

    /// Set the purple accent color.
    pub fn purple(mut self, c: Color) -> Self {
        self.purple = Some(c);
        self
    }

    /// Set the magenta accent color.
    pub fn magenta(mut self, c: Color) -> Self {
        self.magenta = Some(c);
        self
    }

    /// Build the theme.
    ///
    /// If `variant` was not set, it is auto-detected from the background
    /// luminance (dark if luminance <= 0.5). If `contrast` was not set,
    /// it is auto-calculated from the bg/fg WCAG contrast ratio.
    #[must_use]
    pub fn build(self) -> Theme {
        let variant = self.variant.unwrap_or_else(|| {
            if self.bg.luminance() > 0.5 {
                Variant::Light
            } else {
                Variant::Dark
            }
        });
        let contrast = self
            .contrast
            .unwrap_or_else(|| Contrast::from_ratio(self.bg.contrast_ratio(self.fg)));
        Theme {
            name: self.name,
            author: self.author,
            variant,
            contrast,
            bg: self.bg,
            fg: self.fg,
            cursor: self.cursor,
            selection: self.selection,
            line_highlight: self.line_highlight,
            gutter: self.gutter,
            statusbar_bg: self.statusbar_bg,
            statusbar_fg: self.statusbar_fg,
            comment: self.comment,
            keyword: self.keyword,
            string: self.string,
            function: self.function,
            variable: self.variable,
            r#type: self.r#type,
            constant: self.constant,
            operator: self.operator,
            tag: self.tag,
            error: self.error,
            warning: self.warning,
            info: self.info,
            success: self.success,
            red: self.red,
            orange: self.orange,
            yellow: self.yellow,
            green: self.green,
            cyan: self.cyan,
            blue: self.blue,
            purple: self.purple,
            magenta: self.magenta,
        }
    }
}

/// The 16 base16 palette slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
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
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
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
