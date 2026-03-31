//! Convert chromata colors to slint Color values.
//!
//! Run with: `cargo run --example slint_demo --features slint-integration`

#[cfg(feature = "slint-integration")]
fn main() {
    let theme = &chromata::popular::catppuccin::LATTE;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let bg: slint::Color = theme.bg.into();
    let fg: slint::Color = theme.fg.into();

    println!(
        "  bg: slint::Color {{ r: {}, g: {}, b: {} }}",
        bg.red(),
        bg.green(),
        bg.blue()
    );
    println!(
        "  fg: slint::Color {{ r: {}, g: {}, b: {} }}",
        fg.red(),
        fg.green(),
        fg.blue()
    );
}

#[cfg(not(feature = "slint-integration"))]
fn main() {
    eprintln!("This example requires the `slint-integration` feature.");
    eprintln!("Run with: cargo run --example slint_demo --features slint-integration");
}
