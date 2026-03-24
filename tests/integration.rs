use chromata::{Color, Contrast, Variant};

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

// --- New v0.1.0 tests ---

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
    let mut names: Vec<&str> = chromata::popular::THEMES.iter().map(|t| t.name).collect();
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
    let mut names: Vec<&str> = chromata::base16::THEMES.iter().map(|t| t.name).collect();
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
