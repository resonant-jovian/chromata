//! Display theme colors as a styled terminal table using comfy-table.
//!
//! Run with: `cargo run --example comfy_table_demo --features comfy-table-integration`

#[cfg(feature = "comfy-table-integration")]
fn main() {
    use comfy_table::{Cell, Table};

    let theme = &chromata::popular::nord::THEME;

    println!(
        "Theme: {} ({}, {} contrast)\n",
        theme.name, theme.variant, theme.contrast
    );

    let mut table = Table::new();

    // Style header cells with the theme's bg/fg
    let header = vec![
        theme.style_comfy_cell(Cell::new("Role")),
        theme.style_comfy_cell(Cell::new("Hex")),
        theme.style_comfy_cell(Cell::new("RGB")),
    ];
    table.set_header(header);

    for (role, color) in theme.colors() {
        table.add_row(vec![
            Cell::new(role),
            Cell::new(color.to_css_hex()),
            Cell::new(format!("{}, {}, {}", color.r, color.g, color.b)),
        ]);
    }

    println!("{table}");
}

#[cfg(not(feature = "comfy-table-integration"))]
fn main() {
    eprintln!("This example requires the `comfy-table-integration` feature.");
    eprintln!("Run with: cargo run --example comfy_table_demo --features comfy-table-integration");
}
