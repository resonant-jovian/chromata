//! Generate a PNG gradient strip showing all defined colors in a chromata theme.
//!
//! Run with: `cargo run --example image_gradient --features image-integration`

#[cfg(feature = "image-integration")]
fn main() {
    use image::{ImageBuffer, Rgb};

    let theme = &chromata::popular::catppuccin::MOCHA;
    let colors = theme.colors();
    let n = colors.len();

    let width: u32 = 800;
    let height: u32 = 100;
    let stripe_width = width / n as u32;

    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, _y, pixel) in img.enumerate_pixels_mut() {
        let index = (x / stripe_width).min(n as u32 - 1) as usize;
        let (_role, color) = &colors[index];
        *pixel = (*color).into();
    }

    let path = std::env::temp_dir().join("chromata_gradient.png");
    img.save(&path).expect("failed to save PNG");

    println!("Theme: {} ({} colors)", theme.name, n);
    for (role, color) in &colors {
        println!("  {role:<16} {}", color.to_css_hex());
    }
    println!("\nSaved gradient to {}", path.display());
}

#[cfg(not(feature = "image-integration"))]
fn main() {
    eprintln!("This example requires the `image-integration` feature.");
    eprintln!("Run with: cargo run --example image_gradient --features image-integration");
}
