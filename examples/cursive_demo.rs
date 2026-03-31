//! Apply a chromata theme to a cursive palette.
//!
//! Run with: `cargo run --example cursive_demo --features cursive-integration`

#[cfg(feature = "cursive-integration")]
fn main() {
    let theme = &chromata::popular::solarized::DARK;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let mut palette = cursive_core::theme::Palette::default();
    theme.apply_to_cursive_palette(&mut palette);

    println!("Applied theme to cursive palette:");
    println!(
        "  Background:    {:?}",
        palette[cursive_core::theme::PaletteColor::Background]
    );
    println!(
        "  View:          {:?}",
        palette[cursive_core::theme::PaletteColor::View]
    );
    println!(
        "  Primary:       {:?}",
        palette[cursive_core::theme::PaletteColor::Primary]
    );
    println!(
        "  Highlight:     {:?}",
        palette[cursive_core::theme::PaletteColor::Highlight]
    );
    println!(
        "  TitlePrimary:  {:?}",
        palette[cursive_core::theme::PaletteColor::TitlePrimary]
    );
}

#[cfg(not(feature = "cursive-integration"))]
fn main() {
    eprintln!("This example requires the `cursive-integration` feature.");
    eprintln!("Run with: cargo run --example cursive_demo --features cursive-integration");
}
