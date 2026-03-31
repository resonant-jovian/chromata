//! Convert chromata theme background to a wgpu clear color.
//!
//! Run with: `cargo run --example wgpu_demo --features wgpu-integration`

#[cfg(feature = "wgpu-integration")]
fn main() {
    let theme = &chromata::popular::dracula::THEME;

    println!("Theme: {} ({})\n", theme.name, theme.variant);

    let clear: wgpu::Color = theme.bg.into();
    println!(
        "  wgpu clear color: Color {{ r: {:.4}, g: {:.4}, b: {:.4}, a: {:.4} }}",
        clear.r, clear.g, clear.b, clear.a
    );
}

#[cfg(not(feature = "wgpu-integration"))]
fn main() {
    eprintln!("This example requires the `wgpu-integration` feature.");
    eprintln!("Run with: cargo run --example wgpu_demo --features wgpu-integration");
}
