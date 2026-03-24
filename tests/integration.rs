use chromata::{Color, Variant};

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
