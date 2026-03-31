//! Display theme colors in the terminal using crossterm styling.
//!
//! Run with: `cargo run --example crossterm_demo --features crossterm-integration`

#[cfg(feature = "crossterm-integration")]
fn main() {
    use crossterm::style::{Color as CtColor, ResetColor, SetBackgroundColor, SetForegroundColor};

    let theme = &chromata::popular::dracula::THEME;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    // Show bg/fg
    let bg: CtColor = theme.bg.into();
    let fg: CtColor = theme.fg.into();
    println!(
        "  {}{}  background / foreground  {}",
        SetBackgroundColor(bg),
        SetForegroundColor(fg),
        ResetColor
    );

    // Show accent colors
    println!("\nAccent colors:");
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
            let ct: CtColor = c.into();
            println!(
                "  {}  {}  {:<8} {}",
                SetBackgroundColor(ct),
                ResetColor,
                name,
                c.to_css_hex()
            );
        }
    }
}

#[cfg(not(feature = "crossterm-integration"))]
fn main() {
    eprintln!("This example requires the `crossterm-integration` feature.");
    eprintln!("Run with: cargo run --example crossterm_demo --features crossterm-integration");
}
