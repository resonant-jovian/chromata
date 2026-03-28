//! Demonstrate the query APIs: find_by_name, filter_by_variant, filter_by_contrast.
//!
//! Run with: `cargo run --example find_theme`

use chromata::{Contrast, Variant, filter_by_contrast, filter_by_variant, find_by_name};

fn main() {
    // Look up a theme by name
    if let Some(theme) = find_by_name("Gruvbox Dark Hard") {
        println!("Found: {} ({})", theme.name, theme.variant);
        println!("  bg: {}", theme.bg);
        println!("  fg: {}", theme.fg);
        if let Some(kw) = theme.keyword {
            println!("  keyword: {kw}");
        }
    }

    println!();

    // Find all light themes
    let light = filter_by_variant(Variant::Light);
    println!("{} light themes available:", light.len());
    for theme in light.iter().take(10) {
        println!("  {} (contrast: {})", theme.name, theme.contrast);
    }
    if light.len() > 10 {
        println!("  ... and {} more", light.len() - 10);
    }

    println!();

    // Find all high-contrast themes
    let high = filter_by_contrast(Contrast::High);
    println!("{} high-contrast themes:", high.len());
    for theme in high.iter().take(10) {
        println!("  {} ({})", theme.name, theme.variant);
    }
    if high.len() > 10 {
        println!("  ... and {} more", high.len() - 10);
    }
}
