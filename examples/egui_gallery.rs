//! Show how to use chromata themes with egui color types.
//!
//! Run with: `cargo run --example egui_gallery --features egui-integration`

#[cfg(feature = "egui-integration")]
fn main() {
    use chromata::popular;

    let themes = [
        &popular::gruvbox::DARK_HARD,
        &popular::catppuccin::MOCHA,
        &popular::nord::THEME,
        &popular::dracula::THEME,
        &popular::tokyo_night::STORM,
    ];

    for theme in themes {
        let bg: egui::Color32 = theme.bg.into();
        let fg: egui::Color32 = theme.fg.into();

        println!("{}:", theme.name);
        println!("  bg: {:?}", bg);
        println!("  fg: {:?}", fg);

        let colors = theme.colors();
        println!("  {} semantic colors defined", colors.len());
        println!();
    }
}

#[cfg(not(feature = "egui-integration"))]
fn main() {
    eprintln!("This example requires the `egui-integration` feature.");
    eprintln!("Run with: cargo run --example egui_gallery --features egui-integration");
}
