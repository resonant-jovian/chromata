//! Convert chromata colors to tiny-skia premultiplied colors.
//!
//! Run with: `cargo run --example tiny_skia_demo --features tiny-skia-integration`

#[cfg(feature = "tiny-skia-integration")]
fn main() {
    let theme = &chromata::popular::nord::THEME;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let bg: tiny_skia::PremultipliedColorU8 = theme.bg.into();
    let fg: tiny_skia::PremultipliedColorU8 = theme.fg.into();

    println!(
        "  bg: PremultipliedColorU8 {{ r: {}, g: {}, b: {}, a: {} }}",
        bg.red(),
        bg.green(),
        bg.blue(),
        bg.alpha()
    );
    println!(
        "  fg: PremultipliedColorU8 {{ r: {}, g: {}, b: {}, a: {} }}",
        fg.red(),
        fg.green(),
        fg.blue(),
        fg.alpha()
    );
}

#[cfg(not(feature = "tiny-skia-integration"))]
fn main() {
    eprintln!("This example requires the `tiny-skia-integration` feature.");
    eprintln!("Run with: cargo run --example tiny_skia_demo --features tiny-skia-integration");
}
