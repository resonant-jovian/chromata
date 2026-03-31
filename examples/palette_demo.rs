//! Convert chromata colors to palette Srgb for color science operations.
//!
//! Run with: `cargo run --example palette_demo --features palette-integration`

#[cfg(feature = "palette-integration")]
fn main() {
    use palette::{FromColor, Hsl, Srgb};

    let theme = &chromata::popular::nord::THEME;

    println!("Theme: {} ({})\n", theme.name, theme.variant);
    println!(
        "  {:<12} {:>7}  {:>6}  {:>6}  {:>6}",
        "Role", "Hex", "H", "S", "L"
    );
    println!("  {}", "-".repeat(52));

    for (role, color) in theme.colors().iter().take(10) {
        let srgb: Srgb<u8> = (*color).into();
        let srgb_f32: Srgb<f32> = srgb.into_format();
        let hsl: Hsl = Hsl::from_color(srgb_f32);
        println!(
            "  {:<12} {}  {:>5.1}  {:>5.1}%  {:>5.1}%",
            role,
            color.to_css_hex(),
            hsl.hue.into_positive_degrees(),
            hsl.saturation * 100.0,
            hsl.lightness * 100.0,
        );
    }
}

#[cfg(not(feature = "palette-integration"))]
fn main() {
    eprintln!("This example requires the `palette-integration` feature.");
    eprintln!("Run with: cargo run --example palette_demo --features palette-integration");
}
