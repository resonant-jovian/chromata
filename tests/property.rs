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
        prop_assert!(lum >= 0.0 && lum <= 1.0, "luminance out of bounds: {}", lum);
    }
}
