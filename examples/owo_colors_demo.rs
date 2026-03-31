//! Display theme accent colors using owo-colors.
//!
//! Run with: `cargo run --example owo_colors_demo --features owo-colors-integration`

#[cfg(feature = "owo-colors-integration")]
fn main() {
    use owo_colors::OwoColorize;

    let theme = &chromata::popular::catppuccin::MOCHA;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    // Show accent colors as colored text
    let accents: [(&str, Option<chromata::Color>); 8] = [
        ("red", theme.red),
        ("orange", theme.orange),
        ("yellow", theme.yellow),
        ("green", theme.green),
        ("cyan", theme.cyan),
        ("blue", theme.blue),
        ("purple", theme.purple),
        ("magenta", theme.magenta),
    ];

    for (name, color) in accents {
        if let Some(c) = color {
            let rgb: owo_colors::Rgb = c.into();
            let label = format!("  {:<8} {}", name, c.to_css_hex());
            println!("{}", label.color(rgb));
        }
    }
}

#[cfg(not(feature = "owo-colors-integration"))]
fn main() {
    eprintln!("This example requires the `owo-colors-integration` feature.");
    eprintln!("Run with: cargo run --example owo_colors_demo --features owo-colors-integration");
}
