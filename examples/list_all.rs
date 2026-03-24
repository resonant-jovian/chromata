//! List every available theme with its name and variant.
//!
//! Run with: `cargo run --example list_all`

fn main() {
    let themes = chromata::collect_all_themes();
    println!("{} themes available:\n", themes.len());
    for theme in &themes {
        let variant = if theme.is_dark() { "dark" } else { "light" };
        println!(
            "  {name:<30} {variant:<6} bg={bg} fg={fg}",
            name = theme.name,
            variant = variant,
            bg = theme.bg.to_css_hex(),
            fg = theme.fg.to_css_hex(),
        );
    }
}
