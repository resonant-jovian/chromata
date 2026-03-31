//! Generate an SVG color swatch chart from a chromata theme using plotters.
//!
//! Run with: `cargo run --example plotters_chart --features plotters-integration`

#[cfg(feature = "plotters-integration")]
fn main() {
    use plotters::prelude::*;

    let theme = &chromata::popular::gruvbox::DARK_HARD;
    let series_colors = theme.to_plotters_series_colors();

    let path = std::env::temp_dir().join("chromata_chart.svg");
    let root = SVGBackend::new(&path, (640, 480)).into_drawing_area();

    let bg: RGBColor = theme.bg.into();
    let fg: RGBColor = theme.fg.into();

    root.fill(&bg).expect("failed to fill background");

    let color_names = [
        "red", "orange", "yellow", "green", "cyan", "blue", "purple", "magenta",
    ];
    let n = series_colors.len();
    if n == 0 {
        eprintln!("Theme has no accent colors defined.");
        return;
    }

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{} — Accent Colors", theme.name),
            ("sans-serif", 24).into_font().color(&fg),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..n, 0..100)
        .expect("failed to build chart");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(n)
        .x_label_formatter(&|i| color_names.get(*i).unwrap_or(&"?").to_string())
        .label_style(("sans-serif", 14).into_font().color(&fg))
        .draw()
        .expect("failed to draw mesh");

    // Draw one colored bar per accent color
    chart
        .draw_series(series_colors.iter().enumerate().map(|(i, color)| {
            let mut bar = Rectangle::new([(i, 0), (i + 1, 90)], color.filled());
            bar.set_margin(2, 2, 4, 4);
            bar
        }))
        .expect("failed to draw series");

    root.present().expect("failed to write SVG");
    println!("Saved chart to {}", path.display());
}

#[cfg(not(feature = "plotters-integration"))]
fn main() {
    eprintln!("This example requires the `plotters-integration` feature.");
    eprintln!("Run with: cargo run --example plotters_chart --features plotters-integration");
}
