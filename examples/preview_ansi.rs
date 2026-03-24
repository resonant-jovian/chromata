//! Render a theme preview in the terminal using ANSI true-color escape codes.
//!
//! Run with: `cargo run --example preview_ansi`

fn main() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;
    println!("Theme: {}\n", theme.name);

    for (role, color) in theme.colors() {
        let r = color.r;
        let g = color.g;
        let b = color.b;
        // ANSI true-color: ESC[38;2;r;g;bm (foreground) ESC[48;2;r;g;bm (background)
        print!("\x1b[48;2;{r};{g};{b}m  \x1b[0m ");
        println!(
            "\x1b[38;2;{r};{g};{b}m{role:<16}\x1b[0m {hex}",
            hex = color.to_css_hex(),
        );
    }
}
