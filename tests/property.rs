#![allow(clippy::unwrap_used)]
use chromata::Color;
use proptest::prelude::*;

proptest! {
    #[test]
    fn color_hex_roundtrip(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let c = Color::new(r, g, b);
        let hex = c.to_hex();
        let c2 = Color::from_hex(hex);
        prop_assert_eq!(c, c2);
    }

    #[test]
    fn contrast_ratio_is_symmetric(
        r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
        r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
    ) {
        let a = Color::new(r1, g1, b1);
        let b = Color::new(r2, g2, b2);
        let ratio_ab = a.contrast_ratio(b);
        let ratio_ba = b.contrast_ratio(a);
        prop_assert!((ratio_ab - ratio_ba).abs() < 0.001);
    }

    #[test]
    fn luminance_in_bounds(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let c = Color::new(r, g, b);
        let lum = c.luminance();
        prop_assert!((0.0..=1.0).contains(&lum), "luminance out of bounds: {}", lum);
    }

    #[test]
    fn lerp_at_zero_returns_self(
        r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
        r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
    ) {
        let a = Color::new(r1, g1, b1);
        let b = Color::new(r2, g2, b2);
        prop_assert_eq!(a.lerp(b, 0.0), a);
    }

    #[test]
    fn lerp_at_one_returns_other(
        r1 in 0u8..=255, g1 in 0u8..=255, b1 in 0u8..=255,
        r2 in 0u8..=255, g2 in 0u8..=255, b2 in 0u8..=255,
    ) {
        let a = Color::new(r1, g1, b1);
        let b = Color::new(r2, g2, b2);
        let result = a.lerp(b, 1.0);
        // Allow ±1 tolerance for float truncation
        prop_assert!((result.r as i16 - b.r as i16).abs() <= 1);
        prop_assert!((result.g as i16 - b.g as i16).abs() <= 1);
        prop_assert!((result.b as i16 - b.b as i16).abs() <= 1);
    }

    #[test]
    fn to_f32_in_unit_range(r in 0u8..=255, g in 0u8..=255, b in 0u8..=255) {
        let c = Color::new(r, g, b);
        let (rf, gf, bf) = c.to_f32();
        prop_assert!((0.0..=1.0).contains(&rf));
        prop_assert!((0.0..=1.0).contains(&gf));
        prop_assert!((0.0..=1.0).contains(&bf));
    }
}
