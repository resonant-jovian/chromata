//! Convert chromata colors to iced Color values.
//!
//! Run with: `cargo run --example iced_demo --features iced-integration`

#[cfg(feature = "iced-integration")]
fn main() {
    let theme = &chromata::popular::rose_pine::THEME;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let bg: iced_core::Color = theme.bg.into();
    let fg: iced_core::Color = theme.fg.into();

    println!(
        "  bg: Color {{ r: {:.3}, g: {:.3}, b: {:.3}, a: {:.3} }}",
        bg.r, bg.g, bg.b, bg.a
    );
    println!(
        "  fg: Color {{ r: {:.3}, g: {:.3}, b: {:.3}, a: {:.3} }}",
        fg.r, fg.g, fg.b, fg.a
    );
}

#[cfg(not(feature = "iced-integration"))]
fn main() {
    eprintln!("This example requires the `iced-integration` feature.");
    eprintln!("Run with: cargo run --example iced_demo --features iced-integration");
}
