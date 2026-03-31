//! Convert a chromata theme to syntect ThemeSettings.
//!
//! Run with: `cargo run --example syntect_demo --features syntect-integration`

#[cfg(feature = "syntect-integration")]
fn main() {
    let theme = &chromata::popular::gruvbox::DARK_HARD;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let settings = theme.to_syntect_settings();

    if let Some(fg) = settings.foreground {
        println!(
            "  foreground:     rgba({}, {}, {}, {})",
            fg.r, fg.g, fg.b, fg.a
        );
    }
    if let Some(bg) = settings.background {
        println!(
            "  background:     rgba({}, {}, {}, {})",
            bg.r, bg.g, bg.b, bg.a
        );
    }
    if let Some(caret) = settings.caret {
        println!(
            "  caret:          rgba({}, {}, {}, {})",
            caret.r, caret.g, caret.b, caret.a
        );
    }
    if let Some(sel) = settings.selection {
        println!(
            "  selection:      rgba({}, {}, {}, {})",
            sel.r, sel.g, sel.b, sel.a
        );
    }
    if let Some(line) = settings.line_highlight {
        println!(
            "  line_highlight: rgba({}, {}, {}, {})",
            line.r, line.g, line.b, line.a
        );
    }
    if let Some(gutter) = settings.gutter {
        println!(
            "  gutter:         rgba({}, {}, {}, {})",
            gutter.r, gutter.g, gutter.b, gutter.a
        );
    }
}

#[cfg(not(feature = "syntect-integration"))]
fn main() {
    eprintln!("This example requires the `syntect-integration` feature.");
    eprintln!("Run with: cargo run --example syntect_demo --features syntect-integration");
}
