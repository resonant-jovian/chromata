//! Display theme colors using termion terminal styling.
//!
//! Run with: `cargo run --example termion_demo --features termion-integration`

#[cfg(all(unix, feature = "termion-integration"))]
fn main() {
    use termion::color;

    let theme = &chromata::popular::solarized::DARK;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let fg: color::Rgb = theme.fg.into();
    let bg: color::Rgb = theme.bg.into();

    println!(
        "  {}{}  background / foreground  {}{}",
        color::Bg(bg),
        color::Fg(fg),
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    );

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
    for (name, c) in accents {
        if let Some(c) = c {
            let rgb: color::Rgb = c.into();
            println!(
                "  {}  {}{:<8} {}",
                color::Fg(rgb),
                color::Fg(color::Reset),
                name,
                c.to_css_hex(),
            );
        }
    }
}

#[cfg(not(all(unix, feature = "termion-integration")))]
fn main() {
    eprintln!("This example requires unix and the `termion-integration` feature.");
    eprintln!("Run with: cargo run --example termion_demo --features termion-integration");
}
