#![allow(clippy::unwrap_used)]
use chromata::{Color, Contrast, ThemeBuilder, Variant};

#[test]
fn color_roundtrip() {
    let c = Color::new(0x1d, 0x20, 0x21);
    assert_eq!(c.to_hex(), 0x1d2021);
    assert_eq!(Color::from_hex(0x1d2021), c);
}

#[test]
fn color_css_hex() {
    let c = Color::from_hex(0x1d2021);
    assert_eq!(c.to_css_hex(), "#1d2021");
}

#[test]
fn gruvbox_dark_hard_is_dark() {
    let t = &chromata::popular::gruvbox::DARK_HARD;
    assert!(t.is_dark());
    assert_eq!(t.variant, Variant::Dark);
}

#[test]
fn gruvbox_light_is_light() {
    let t = &chromata::popular::gruvbox::LIGHT;
    assert!(!t.is_dark());
    assert_eq!(t.variant, Variant::Light);
}

#[test]
fn all_themes_have_distinct_bg_fg() {
    for theme in chromata::collect_all_themes() {
        assert_ne!(
            theme.bg, theme.fg,
            "Theme '{}' has identical bg and fg",
            theme.name
        );
    }
}

#[test]
fn all_themes_have_minimum_syntax_colors() {
    for theme in chromata::collect_all_themes() {
        let syntax_count = [
            theme.comment,
            theme.keyword,
            theme.string,
            theme.function,
            theme.variable,
            theme.r#type,
            theme.constant,
        ]
        .iter()
        .filter(|c| c.is_some())
        .count();
        assert!(
            syntax_count >= 4,
            "Theme '{}' has only {} syntax colors (need >= 4)",
            theme.name,
            syntax_count
        );
    }
}

#[test]
fn accent_returns_a_color() {
    let t = &chromata::popular::gruvbox::DARK_HARD;
    let accent = t.accent();
    // Gruvbox has blue defined, so accent should be blue
    assert_eq!(accent, t.blue.unwrap());
}

#[test]
fn contrast_ratio_is_symmetric() {
    let a = Color::from_hex(0x000000);
    let b = Color::from_hex(0xffffff);
    let ratio_ab = a.contrast_ratio(b);
    let ratio_ba = b.contrast_ratio(a);
    assert!((ratio_ab - ratio_ba).abs() < 0.001);
    // Black/white should have ~21:1 contrast
    assert!(ratio_ab > 20.0);
}

// Thresholds intentionally overlap in the 0.15–0.4 range because some
// themes have mid-range backgrounds that could reasonably be either variant.
#[test]
fn variant_matches_bg_luminance() {
    for theme in chromata::collect_all_themes() {
        let lum = theme.bg.luminance();
        match theme.variant {
            Variant::Dark => assert!(
                lum < 0.4,
                "Theme '{}' is marked Dark but bg luminance is {:.3} (expected < 0.4)",
                theme.name,
                lum
            ),
            Variant::Light => assert!(
                lum > 0.15,
                "Theme '{}' is marked Light but bg luminance is {:.3} (expected > 0.15)",
                theme.name,
                lum
            ),
        }
    }
}

#[test]
fn no_duplicate_theme_names_in_popular() {
    let mut names: Vec<&str> = chromata::popular::THEMES
        .iter()
        .map(|t| t.name.as_ref())
        .collect();
    names.sort();
    for window in names.windows(2) {
        assert_ne!(
            window[0], window[1],
            "Duplicate theme name in popular: '{}'",
            window[0]
        );
    }
}

#[cfg(feature = "base16")]
#[test]
fn no_duplicate_theme_names_in_base16() {
    let mut names: Vec<&str> = chromata::base16::THEMES
        .iter()
        .map(|t| t.name.as_ref())
        .collect();
    names.sort();
    for window in names.windows(2) {
        assert_ne!(
            window[0], window[1],
            "Duplicate theme name in base16: '{}'",
            window[0]
        );
    }
}

#[test]
fn contrast_field_matches_calculation() {
    let mut failures = Vec::new();
    for theme in chromata::collect_all_themes() {
        let ratio = theme.bg.contrast_ratio(theme.fg);
        let expected = if ratio >= 10.0 {
            Contrast::High
        } else if ratio >= 4.5 {
            Contrast::Normal
        } else {
            Contrast::Low
        };
        if theme.contrast != expected {
            failures.push(format!(
                "  {}: has {:?} but ratio is {:.2} (expected {:?})",
                theme.name, theme.contrast, ratio, expected
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "Contrast mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn popular_has_expected_count() {
    let themes = chromata::collect_all_themes();
    assert!(
        themes.len() >= 40,
        "Expected at least 40 popular themes, got {}",
        themes.len()
    );
}

#[cfg(feature = "base16")]
#[test]
fn base16_has_expected_count() {
    assert!(
        chromata::base16::THEMES.len() >= 300,
        "Expected at least 300 base16 themes, got {}",
        chromata::base16::THEMES.len()
    );
}

// --- base24 tests ---

#[cfg(feature = "base24")]
#[test]
fn base24_has_expected_count() {
    assert!(
        chromata::base24::THEMES.len() >= 150,
        "Expected at least 150 base24 themes, got {}",
        chromata::base24::THEMES.len()
    );
}

#[cfg(feature = "base24")]
#[test]
fn no_duplicate_theme_names_in_base24() {
    let mut names: Vec<&str> = chromata::base24::THEMES
        .iter()
        .map(|t| t.name.as_ref())
        .collect();
    names.sort();
    for window in names.windows(2) {
        assert_ne!(
            window[0], window[1],
            "Duplicate theme name in base24: '{}'",
            window[0]
        );
    }
}

// --- emacs tests ---

#[cfg(feature = "emacs")]
#[test]
fn emacs_has_expected_count() {
    assert!(
        chromata::emacs::THEMES.len() >= 80,
        "Expected at least 80 emacs themes, got {}",
        chromata::emacs::THEMES.len()
    );
}

#[cfg(feature = "emacs")]
#[test]
fn no_duplicate_theme_names_in_emacs() {
    let mut names: Vec<&str> = chromata::emacs::THEMES
        .iter()
        .map(|t| t.name.as_ref())
        .collect();
    names.sort();
    for window in names.windows(2) {
        assert_ne!(
            window[0], window[1],
            "Duplicate theme name in emacs: '{}'",
            window[0]
        );
    }
}

#[cfg(feature = "emacs")]
#[test]
fn emacs_all_have_bg_fg() {
    for theme in chromata::emacs::THEMES {
        assert_ne!(
            theme.bg, theme.fg,
            "Emacs theme '{}' has same bg and fg",
            theme.name
        );
    }
}

// --- vim tests ---

#[cfg(feature = "vim")]
#[test]
fn vim_has_expected_count() {
    assert!(
        chromata::vim::THEMES.len() >= 300,
        "Expected at least 300 vim themes, got {}",
        chromata::vim::THEMES.len()
    );
}

#[cfg(feature = "vim")]
#[test]
fn vim_all_have_bg_fg() {
    for theme in chromata::vim::THEMES {
        assert_ne!(
            theme.bg, theme.fg,
            "Vim theme '{}' has same bg and fg",
            theme.name
        );
    }
}

#[cfg(feature = "vim")]
#[test]
fn vim_no_duplicate_names() {
    let mut names: Vec<&str> = chromata::vim::THEMES
        .iter()
        .map(|t| t.name.as_ref())
        .collect();
    names.sort();
    for window in names.windows(2) {
        assert_ne!(window[0], window[1], "Duplicate vim theme: '{}'", window[0]);
    }
}

// --- base24 bg/fg test ---

#[cfg(feature = "base24")]
#[test]
fn base24_all_have_bg_fg() {
    for theme in chromata::base24::THEMES {
        assert_ne!(
            theme.bg, theme.fg,
            "Base24 theme '{}' has same bg and fg",
            theme.name
        );
    }
}

// --- per-collection contrast validation ---

#[cfg(feature = "emacs")]
#[test]
fn emacs_contrast_matches_calculation() {
    let mut failures = Vec::new();
    for theme in chromata::emacs::THEMES {
        let ratio = theme.bg.contrast_ratio(theme.fg);
        let expected = if ratio >= 10.0 {
            Contrast::High
        } else if ratio >= 4.5 {
            Contrast::Normal
        } else {
            Contrast::Low
        };
        if theme.contrast != expected {
            failures.push(format!(
                "  {}: has {:?} but ratio is {:.2} (expected {:?})",
                theme.name, theme.contrast, ratio, expected
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "Emacs contrast mismatches:\n{}",
        failures.join("\n")
    );
}

#[cfg(feature = "vim")]
#[test]
fn vim_contrast_matches_calculation() {
    let mut failures = Vec::new();
    for theme in chromata::vim::THEMES {
        let ratio = theme.bg.contrast_ratio(theme.fg);
        let expected = if ratio >= 10.0 {
            Contrast::High
        } else if ratio >= 4.5 {
            Contrast::Normal
        } else {
            Contrast::Low
        };
        if theme.contrast != expected {
            failures.push(format!(
                "  {}: has {:?} but ratio is {:.2} (expected {:?})",
                theme.name, theme.contrast, ratio, expected
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "Vim contrast mismatches:\n{}",
        failures.join("\n")
    );
}

// --- Query API tests ---

#[test]
fn find_by_name_returns_known_theme() {
    let theme = chromata::find_by_name("Gruvbox Dark Hard");
    assert!(theme.is_some());
    assert_eq!(theme.unwrap().name, "Gruvbox Dark Hard");
}

#[test]
fn find_by_name_returns_none_for_unknown() {
    assert!(chromata::find_by_name("Nonexistent Theme XYZ").is_none());
}

#[test]
fn filter_by_variant_returns_only_dark() {
    let dark = chromata::filter_by_variant(Variant::Dark);
    for t in &dark {
        assert_eq!(t.variant, Variant::Dark, "'{}' is not Dark", t.name);
    }
    assert!(!dark.is_empty());
}

#[test]
fn filter_by_contrast_returns_correct_level() {
    let high = chromata::filter_by_contrast(Contrast::High);
    for t in &high {
        assert_eq!(
            t.contrast,
            Contrast::High,
            "'{}' is not High contrast",
            t.name
        );
    }
}

// --- Serde tests ---

#[cfg(feature = "serde-support")]
#[test]
fn serde_color_roundtrip() {
    let c = Color::new(0x1d, 0x20, 0x21);
    let json = serde_json::to_string(&c).unwrap();
    let c2: Color = serde_json::from_str(&json).unwrap();
    assert_eq!(c, c2);
}

#[cfg(feature = "serde-support")]
#[test]
fn serde_theme_serializes() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;
    let json = serde_json::to_string_pretty(theme).unwrap();
    assert!(json.contains("Gruvbox Dark Hard"));
    assert!(json.contains("bg"));
    assert!(json.contains("fg"));
}

// --- Display impl tests ---

#[test]
fn color_display_format() {
    let c = Color::from_hex(0x1d2021);
    assert_eq!(format!("{c}"), "#1d2021");
}

#[test]
fn variant_display() {
    assert_eq!(format!("{}", Variant::Dark), "Dark");
    assert_eq!(format!("{}", Variant::Light), "Light");
}

#[test]
fn contrast_display() {
    assert_eq!(format!("{}", Contrast::High), "High");
    assert_eq!(format!("{}", Contrast::Normal), "Normal");
    assert_eq!(format!("{}", Contrast::Low), "Low");
}

// --- Reference value tests (spot-check popular theme colors) ---

#[test]
fn gruvbox_dark_hard_reference_colors() {
    let t = &chromata::popular::gruvbox::DARK_HARD;
    assert_eq!(t.bg, Color::from_hex(0x1d2021));
    assert_eq!(t.fg, Color::from_hex(0xd5c4a1));
    assert_eq!(t.name, "Gruvbox Dark Hard");
    assert_eq!(t.variant, Variant::Dark);
}

#[test]
fn gruvbox_light_reference_colors() {
    let t = &chromata::popular::gruvbox::LIGHT;
    assert_eq!(t.bg, Color::from_hex(0xfbf1c7));
    assert_eq!(t.fg, Color::from_hex(0x504945));
    assert_eq!(t.variant, Variant::Light);
}

#[test]
fn catppuccin_mocha_reference_colors() {
    let theme = chromata::find_by_name("Catppuccin Mocha");
    assert!(theme.is_some());
    let t = theme.unwrap();
    assert_eq!(t.bg, Color::from_hex(0x1e1e2e));
    assert_eq!(t.fg, Color::from_hex(0xcdd6f4));
    assert_eq!(t.variant, Variant::Dark);
}

#[test]
fn nord_reference_colors() {
    let theme = chromata::find_by_name("Nord");
    assert!(theme.is_some());
    let t = theme.unwrap();
    assert_eq!(t.bg, Color::from_hex(0x2e3440));
    assert_eq!(t.fg, Color::from_hex(0xd8dee9));
    assert_eq!(t.variant, Variant::Dark);
}

// --- From/Into impls for Color ---

#[test]
fn color_from_u32() {
    let c: Color = 0x1d2021u32.into();
    assert_eq!(c, Color::from_hex(0x1d2021));
}

#[test]
fn color_from_tuple() {
    let c: Color = (0x1d, 0x20, 0x21).into();
    assert_eq!(c, Color::new(0x1d, 0x20, 0x21));
}

#[test]
fn color_from_str() {
    let c: Color = "#1d2021".parse().unwrap();
    assert_eq!(c, Color::from_hex(0x1d2021));

    let c: Color = "1d2021".parse().unwrap();
    assert_eq!(c, Color::from_hex(0x1d2021));

    assert!("nope".parse::<Color>().is_err());
}

// --- 3-digit CSS hex ---

#[test]
fn color_css_hex_3digit() {
    let c = Color::from_css_hex("#FFF").unwrap();
    assert_eq!(c, Color::new(255, 255, 255));

    let c = Color::from_css_hex("000").unwrap();
    assert_eq!(c, Color::new(0, 0, 0));

    let c = Color::from_css_hex("#ABC").unwrap();
    assert_eq!(c, Color::new(0xAA, 0xBB, 0xCC));
}

#[test]
fn color_css_hex_mixed_case() {
    let c = Color::from_css_hex("#A1b2C3").unwrap();
    assert_eq!(c, Color::new(0xA1, 0xB2, 0xC3));
}

// --- FromStr for Variant ---

#[test]
fn variant_from_str() {
    assert_eq!("Dark".parse::<Variant>().unwrap(), Variant::Dark);
    assert_eq!("dark".parse::<Variant>().unwrap(), Variant::Dark);
    assert_eq!("DARK".parse::<Variant>().unwrap(), Variant::Dark);
    assert_eq!("Light".parse::<Variant>().unwrap(), Variant::Light);
    assert_eq!("light".parse::<Variant>().unwrap(), Variant::Light);
    assert!("invalid".parse::<Variant>().is_err());
}

// --- FromStr for Contrast ---

#[test]
fn contrast_from_str() {
    assert_eq!("High".parse::<Contrast>().unwrap(), Contrast::High);
    assert_eq!("high".parse::<Contrast>().unwrap(), Contrast::High);
    assert_eq!("Normal".parse::<Contrast>().unwrap(), Contrast::Normal);
    assert_eq!("Low".parse::<Contrast>().unwrap(), Contrast::Low);
    assert!("invalid".parse::<Contrast>().is_err());
}

// --- Default impls ---

#[test]
fn variant_default_is_dark() {
    assert_eq!(Variant::default(), Variant::Dark);
}

#[test]
fn contrast_default_is_normal() {
    assert_eq!(Contrast::default(), Contrast::Normal);
}

// --- Contrast::from_ratio ---

#[test]
fn contrast_from_ratio_boundaries() {
    assert_eq!(Contrast::from_ratio(10.0), Contrast::High);
    assert_eq!(Contrast::from_ratio(10.1), Contrast::High);
    assert_eq!(Contrast::from_ratio(9.99), Contrast::Normal);
    assert_eq!(Contrast::from_ratio(4.5), Contrast::Normal);
    assert_eq!(Contrast::from_ratio(4.49), Contrast::Low);
    assert_eq!(Contrast::from_ratio(1.0), Contrast::Low);
    assert_eq!(Contrast::from_ratio(21.0), Contrast::High);
}

// --- ThemeBuilder ---

#[test]
fn theme_builder_basic() {
    let theme = ThemeBuilder::new(
        "Test Theme",
        "Test Author",
        Color::new(0, 0, 0),
        Color::new(255, 255, 255),
    )
    .build();
    assert_eq!(theme.name, "Test Theme");
    assert_eq!(theme.author, "Test Author");
    assert_eq!(theme.bg, Color::new(0, 0, 0));
    assert_eq!(theme.fg, Color::new(255, 255, 255));
}

#[test]
fn theme_builder_auto_variant() {
    let dark = ThemeBuilder::new("D", "", Color::new(0, 0, 0), Color::new(255, 255, 255)).build();
    assert_eq!(dark.variant, Variant::Dark);

    let light = ThemeBuilder::new("L", "", Color::new(255, 255, 255), Color::new(0, 0, 0)).build();
    assert_eq!(light.variant, Variant::Light);
}

#[test]
fn theme_builder_auto_contrast() {
    let theme = ThemeBuilder::new("T", "", Color::new(0, 0, 0), Color::new(255, 255, 255)).build();
    assert_eq!(theme.contrast, Contrast::High);
}

#[test]
fn theme_builder_with_string_name() {
    let name = String::from("Dynamic Theme");
    let theme = ThemeBuilder::new(
        name,
        String::from("Author"),
        Color::new(0, 0, 0),
        Color::new(200, 200, 200),
    )
    .build();
    assert_eq!(theme.name, "Dynamic Theme");
}

#[test]
fn theme_builder_via_theme() {
    let theme = chromata::Theme::builder(
        "My Theme",
        "Me",
        Color::new(0, 0, 0),
        Color::new(255, 255, 255),
    )
    .keyword(Color::from_hex(0xff79c6))
    .build();
    assert_eq!(theme.keyword, Some(Color::from_hex(0xff79c6)));
}

// --- Lerp edge cases ---

#[test]
fn lerp_intermediate_values() {
    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);

    let quarter = black.lerp(white, 0.25);
    assert_eq!(quarter, Color::new(63, 63, 63));

    let half = black.lerp(white, 0.5);
    assert_eq!(half, Color::new(127, 127, 127));

    let three_quarter = black.lerp(white, 0.75);
    assert_eq!(three_quarter, Color::new(191, 191, 191));
}

#[test]
fn lerp_clamps_out_of_bounds() {
    let a = Color::new(100, 100, 100);
    let b = Color::new(200, 200, 200);
    assert_eq!(a.lerp(b, -1.0), a);
    assert_eq!(a.lerp(b, 2.0), b);
}

// --- Serde roundtrip (Theme deserialize) ---

#[cfg(feature = "serde-support")]
#[test]
fn serde_theme_roundtrip() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;
    let json = serde_json::to_string(theme).unwrap();
    let deserialized: chromata::Theme = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.name, theme.name);
    assert_eq!(deserialized.bg, theme.bg);
    assert_eq!(deserialized.fg, theme.fg);
    assert_eq!(deserialized.variant, theme.variant);
    assert_eq!(deserialized.contrast, theme.contrast);
    assert_eq!(deserialized.keyword, theme.keyword);
}

#[cfg(feature = "serde-support")]
#[test]
fn serde_deserialize_minimal_json() {
    let json = r#"{
        "name": "Minimal",
        "author": "Test",
        "variant": "Dark",
        "contrast": "Normal",
        "bg": {"r": 0, "g": 0, "b": 0},
        "fg": {"r": 255, "g": 255, "b": 255}
    }"#;
    let theme: chromata::Theme = serde_json::from_str(json).unwrap();
    assert_eq!(theme.name, "Minimal");
    assert_eq!(theme.bg, Color::new(0, 0, 0));
    assert!(theme.keyword.is_none());
    assert!(theme.cursor.is_none());
    assert!(theme.red.is_none());
}

// --- Invalid hex lengths ---

#[test]
fn color_css_hex_rejects_invalid_lengths() {
    // 4-digit (#RGBA) not supported
    assert!(Color::from_css_hex("#ABCD").is_none());
    // 8-digit (#RRGGBBAA) not supported
    assert!(Color::from_css_hex("#AABBCCDD").is_none());
    // 1-digit
    assert!(Color::from_css_hex("#A").is_none());
    // 2-digit
    assert!(Color::from_css_hex("#AB").is_none());
    // 5-digit
    assert!(Color::from_css_hex("#ABCDE").is_none());
    // empty
    assert!(Color::from_css_hex("").is_none());
    assert!(Color::from_css_hex("#").is_none());
}

// --- Builder override tests ---

#[test]
fn theme_builder_override_variant() {
    // Dark bg but explicitly set to Light
    let theme = ThemeBuilder::new("T", "", Color::new(0, 0, 0), Color::new(255, 255, 255))
        .variant(Variant::Light)
        .build();
    assert_eq!(theme.variant, Variant::Light);
}

#[test]
fn theme_builder_override_contrast() {
    // High contrast bg/fg but explicitly set to Low
    let theme = ThemeBuilder::new("T", "", Color::new(0, 0, 0), Color::new(255, 255, 255))
        .contrast(Contrast::Low)
        .build();
    assert_eq!(theme.contrast, Contrast::Low);
}

// --- Error type tests ---

#[test]
fn parse_errors_have_display() {
    use std::string::ToString;
    let e: chromata::ParseColorError = "#xyz".parse::<Color>().unwrap_err();
    assert!(!e.to_string().is_empty());

    let e: chromata::ParseVariantError = "nope".parse::<Variant>().unwrap_err();
    assert!(!e.to_string().is_empty());

    let e: chromata::ParseContrastError = "nope".parse::<Contrast>().unwrap_err();
    assert!(!e.to_string().is_empty());
}

#[test]
fn parse_errors_are_std_error() {
    fn assert_error<E: std::error::Error>() {}
    assert_error::<chromata::ParseColorError>();
    assert_error::<chromata::ParseVariantError>();
    assert_error::<chromata::ParseContrastError>();
}

// --- Reverse From conversions ---

#[test]
fn color_into_u32() {
    let c = Color::from_hex(0x1d2021);
    let v: u32 = c.into();
    assert_eq!(v, 0x1d2021);
}

#[test]
fn color_into_tuple() {
    let c = Color::new(29, 32, 33);
    let (r, g, b): (u8, u8, u8) = c.into();
    assert_eq!((r, g, b), (29, 32, 33));
}

// --- 3-digit hex with invalid chars ---

#[test]
fn color_css_hex_3digit_invalid_chars() {
    assert!(Color::from_css_hex("#XYZ").is_none());
    assert!(Color::from_css_hex("GHI").is_none());
}

// --- Serde forward-compatibility ---

#[cfg(feature = "serde-support")]
#[test]
fn serde_ignores_unknown_fields() {
    let json = r#"{
        "name": "Test", "author": "A", "variant": "Dark", "contrast": "Normal",
        "bg": {"r": 0, "g": 0, "b": 0}, "fg": {"r": 255, "g": 255, "b": 255},
        "future_field": "some_value", "border": {"r": 128, "g": 128, "b": 128}
    }"#;
    let theme: chromata::Theme = serde_json::from_str(json).unwrap();
    assert_eq!(theme.name, "Test");
    assert_eq!(theme.bg, Color::new(0, 0, 0));
}

// --- accent() fallback to fg ---

#[test]
fn accent_fallback_to_fg() {
    let theme = ThemeBuilder::new("T", "", Color::new(0, 0, 0), Color::new(200, 200, 200)).build();
    // No accent colors set, so accent() should return fg
    assert_eq!(theme.accent(), Color::new(200, 200, 200));
}

// --- colors() count ---

#[test]
fn colors_count_matches_defined_fields() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;
    let colors = theme.colors();
    // bg + fg are always present, plus all Some(...) optional fields
    assert!(colors.len() >= 2);
    // Gruvbox Dark Hard has all fields defined (bg + fg + 27 optional = 29)
    assert_eq!(colors.len(), 29);
}
