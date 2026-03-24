//! Generate CSS custom properties from a chromata theme.
//!
//! Run with: `cargo run --example export_css`

fn main() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;
    println!(":root {{");
    for (role, color) in theme.colors() {
        println!(
            "  --chromata-{}: {};",
            role.replace('_', "-"),
            color.to_css_hex()
        );
    }
    println!("}}");
}
