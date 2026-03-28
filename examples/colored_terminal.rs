//! Display a themed code snippet in the terminal using the colored crate.
//!
//! Run with: `cargo run --example colored_terminal --features colored-integration`

#[cfg(feature = "colored-integration")]
fn main() {
    use colored::Colorize;

    let theme = &chromata::popular::dracula::THEME;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    // Show a block with the theme's bg/fg applied
    println!("{}", theme.colorize("  background / foreground  "));
    println!();

    // Simulate a small code snippet with syntax-colored parts
    if let Some(kw) = theme.keyword {
        print!("{}", "fn ".truecolor(kw.r, kw.g, kw.b));
    }
    if let Some(func) = theme.function {
        print!("{}", "main".truecolor(func.r, func.g, func.b));
    }
    println!("() {{");

    if let Some(kw) = theme.keyword {
        print!("    {}", "let ".truecolor(kw.r, kw.g, kw.b));
    }
    print!("msg");
    print!(" = ");
    if let Some(s) = theme.string {
        print!("{}", "\"hello, chromata\"".truecolor(s.r, s.g, s.b));
    }
    println!(";");

    if let Some(comment) = theme.comment {
        println!(
            "    {}",
            "// print the message".truecolor(comment.r, comment.g, comment.b)
        );
    }

    if let Some(func) = theme.function {
        print!("    {}", "println!".truecolor(func.r, func.g, func.b));
    }
    println!("(\"{{}}\", msg);");

    println!("}}");

    // Show the accent palette
    println!("\nAccent colors:");
    let accent_fields: [(&str, Option<chromata::Color>); 8] = [
        ("red", theme.red),
        ("orange", theme.orange),
        ("yellow", theme.yellow),
        ("green", theme.green),
        ("cyan", theme.cyan),
        ("blue", theme.blue),
        ("purple", theme.purple),
        ("magenta", theme.magenta),
    ];
    for (name, color) in accent_fields {
        if let Some(c) = color {
            println!(
                "  {} {}",
                "  ".on_truecolor(c.r, c.g, c.b),
                name.truecolor(c.r, c.g, c.b),
            );
        }
    }
}

#[cfg(not(feature = "colored-integration"))]
fn main() {
    eprintln!("This example requires the `colored-integration` feature.");
    eprintln!("Run with: cargo run --example colored_terminal --features colored-integration");
}
