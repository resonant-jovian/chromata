//! Convert chromata colors to macroquad Color values.
//!
//! Run with: `cargo run --example macroquad_demo --features macroquad-integration`

#[cfg(feature = "macroquad-integration")]
fn main() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let bg: macroquad::color::Color = theme.bg.into();
    let fg: macroquad::color::Color = theme.fg.into();

    println!(
        "  bg: Color {{ r: {:.3}, g: {:.3}, b: {:.3}, a: {:.3} }}",
        bg.r, bg.g, bg.b, bg.a
    );
    println!(
        "  fg: Color {{ r: {:.3}, g: {:.3}, b: {:.3}, a: {:.3} }}",
        fg.r, fg.g, fg.b, fg.a
    );
}

#[cfg(not(feature = "macroquad-integration"))]
fn main() {
    eprintln!("This example requires the `macroquad-integration` feature.");
    eprintln!("Run with: cargo run --example macroquad_demo --features macroquad-integration");
}
