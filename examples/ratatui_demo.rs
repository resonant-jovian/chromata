//! A simple demo showing how to use chromata themes with ratatui styling.
//!
//! Run with: `cargo run --example ratatui_demo --features ratatui-integration`

#[cfg(feature = "ratatui-integration")]
fn main() {
    use chromata::popular::gruvbox;

    let theme = gruvbox::DARK_HARD;
    let bg: ratatui::style::Color = theme.bg.into();
    let fg: ratatui::style::Color = theme.fg.into();

    println!("Ratatui colors from {} theme:", theme.name);
    println!("  bg: {:?}", bg);
    println!("  fg: {:?}", fg);

    // Build a ratatui Style from the theme
    let style = theme.to_ratatui_style();
    println!("  style: {:?}", style);

    // Show all available accent colors
    if let Some(keyword) = theme.keyword {
        let kw_color: ratatui::style::Color = keyword.into();
        println!("  keyword: {:?}", kw_color);
    }
    if let Some(string) = theme.string {
        let str_color: ratatui::style::Color = string.into();
        println!("  string: {:?}", str_color);
    }
}

#[cfg(not(feature = "ratatui-integration"))]
fn main() {
    eprintln!("This example requires the `ratatui-integration` feature.");
    eprintln!("Run with: cargo run --example ratatui_demo --features ratatui-integration");
}
