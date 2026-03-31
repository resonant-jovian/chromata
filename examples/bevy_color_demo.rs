//! Convert chromata colors to bevy_color Srgba values.
//!
//! Run with: `cargo run --example bevy_color_demo --features bevy-color-integration`

#[cfg(feature = "bevy-color-integration")]
fn main() {
    let theme = &chromata::popular::tokyo_night::STORM;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let bg: bevy_color::Srgba = theme.bg.into();
    let fg: bevy_color::Srgba = theme.fg.into();

    println!(
        "  bg: Srgba({:.3}, {:.3}, {:.3}, {:.3})",
        bg.red, bg.green, bg.blue, bg.alpha
    );
    println!(
        "  fg: Srgba({:.3}, {:.3}, {:.3}, {:.3})",
        fg.red, fg.green, fg.blue, fg.alpha
    );

    if let Some(kw) = theme.keyword {
        let c: bevy_color::Srgba = kw.into();
        println!(
            "  kw: Srgba({:.3}, {:.3}, {:.3}, {:.3})",
            c.red, c.green, c.blue, c.alpha
        );
    }
}

#[cfg(not(feature = "bevy-color-integration"))]
fn main() {
    eprintln!("This example requires the `bevy-color-integration` feature.");
    eprintln!("Run with: cargo run --example bevy_color_demo --features bevy-color-integration");
}
