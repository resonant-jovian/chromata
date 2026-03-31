#![allow(clippy::unwrap_used)]

// --- Color conversion tests (original 4 integrations) ---

#[cfg(feature = "ratatui-integration")]
#[test]
fn ratatui_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: ratatui::style::Color = c.into();
    assert_eq!(p, ratatui::style::Color::Rgb(128, 64, 32));
}

#[cfg(feature = "egui-integration")]
#[test]
fn egui_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: egui::Color32 = c.into();
    assert_eq!(p, egui::Color32::from_rgb(128, 64, 32));
}

#[cfg(feature = "crossterm-integration")]
#[test]
fn crossterm_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: crossterm::style::Color = c.into();
    assert_eq!(
        p,
        crossterm::style::Color::Rgb {
            r: 128,
            g: 64,
            b: 32
        }
    );
}

#[cfg(feature = "iced-integration")]
#[test]
fn iced_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: iced_core::Color = c.into();
    assert!((p.r - 128.0 / 255.0).abs() < 0.01);
    assert!((p.g - 64.0 / 255.0).abs() < 0.01);
    assert!((p.b - 32.0 / 255.0).abs() < 0.01);
    assert!((p.a - 1.0).abs() < 0.01);
}

// --- Color conversion tests (14 new integrations) ---

#[cfg(feature = "plotters-integration")]
#[test]
fn plotters_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: plotters::style::RGBColor = c.into();
    assert_eq!(p.0, 128);
    assert_eq!(p.1, 64);
    assert_eq!(p.2, 32);
}

#[cfg(feature = "image-integration")]
#[test]
fn image_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: image::Rgb<u8> = c.into();
    assert_eq!(p.0[0], 128);
    assert_eq!(p.0[1], 64);
    assert_eq!(p.0[2], 32);
}

#[cfg(feature = "palette-integration")]
#[test]
fn palette_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: palette::Srgb<u8> = c.into();
    assert_eq!(p.red, 128);
    assert_eq!(p.green, 64);
    assert_eq!(p.blue, 32);
}

#[cfg(feature = "bevy-color-integration")]
#[test]
fn bevy_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: bevy_color::Srgba = c.into();
    assert!((p.red - 128.0 / 255.0).abs() < 0.01);
    assert!((p.green - 64.0 / 255.0).abs() < 0.01);
    assert!((p.blue - 32.0 / 255.0).abs() < 0.01);
    assert!((p.alpha - 1.0).abs() < 0.01);
}

#[cfg(feature = "macroquad-integration")]
#[test]
fn macroquad_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: macroquad::color::Color = c.into();
    assert!((p.r - 128.0 / 255.0).abs() < 0.01);
    assert!((p.g - 64.0 / 255.0).abs() < 0.01);
    assert!((p.b - 32.0 / 255.0).abs() < 0.01);
    assert!((p.a - 1.0).abs() < 0.01);
}

#[cfg(feature = "tiny-skia-integration")]
#[test]
fn tiny_skia_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: tiny_skia::PremultipliedColorU8 = c.into();
    assert_eq!(p.red(), 128);
    assert_eq!(p.green(), 64);
    assert_eq!(p.blue(), 32);
    assert_eq!(p.alpha(), 255);
}

#[cfg(feature = "wgpu-integration")]
#[test]
fn wgpu_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: wgpu::Color = c.into();
    assert!((p.r - 128.0 / 255.0).abs() < 0.01);
    assert!((p.g - 64.0 / 255.0).abs() < 0.01);
    assert!((p.b - 32.0 / 255.0).abs() < 0.01);
    assert!((p.a - 1.0).abs() < 0.01);
}

#[cfg(feature = "colored-integration")]
#[test]
fn colored_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: colored::Color = c.into();
    match p {
        colored::Color::TrueColor { r, g, b } => {
            assert_eq!(r, 128);
            assert_eq!(g, 64);
            assert_eq!(b, 32);
        }
        other => panic!("Expected TrueColor, got {:?}", other),
    }
}

#[cfg(feature = "owo-colors-integration")]
#[test]
fn owo_colors_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: owo_colors::Rgb = c.into();
    assert_eq!(p.0, 128);
    assert_eq!(p.1, 64);
    assert_eq!(p.2, 32);
}

#[cfg(all(unix, feature = "termion-integration"))]
#[test]
fn termion_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: termion::color::Rgb = c.into();
    assert_eq!(p.0, 128);
    assert_eq!(p.1, 64);
    assert_eq!(p.2, 32);
}

#[cfg(feature = "cursive-integration")]
#[test]
fn cursive_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: cursive_core::theme::Color = c.into();
    match p {
        cursive_core::theme::Color::Rgb(r, g, b) => {
            assert_eq!(r, 128);
            assert_eq!(g, 64);
            assert_eq!(b, 32);
        }
        other => panic!("Expected Rgb, got {:?}", other),
    }
}

#[cfg(feature = "comfy-table-integration")]
#[test]
fn comfy_table_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: comfy_table::Color = c.into();
    match p {
        comfy_table::Color::Rgb { r, g, b } => {
            assert_eq!(r, 128);
            assert_eq!(g, 64);
            assert_eq!(b, 32);
        }
        other => panic!("Expected Rgb, got {:?}", other),
    }
}

#[cfg(feature = "syntect-integration")]
#[test]
fn syntect_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: syntect::highlighting::Color = c.into();
    assert_eq!(p.r, 128);
    assert_eq!(p.g, 64);
    assert_eq!(p.b, 32);
    assert_eq!(p.a, 255);
}

#[cfg(feature = "slint-integration")]
#[test]
fn slint_color_conversion() {
    let c = chromata::Color::new(128, 64, 32);
    let p: slint::Color = c.into();
    assert_eq!(p.red(), 128);
    assert_eq!(p.green(), 64);
    assert_eq!(p.blue(), 32);
    assert_eq!(p.alpha(), 255);
}

// --- Convenience method tests ---

#[cfg(feature = "plotters-integration")]
#[test]
fn plotters_series_colors() {
    let colors = chromata::popular::gruvbox::DARK_HARD.to_plotters_series_colors();
    assert!(!colors.is_empty());
}

#[cfg(feature = "syntect-integration")]
#[test]
fn syntect_theme_settings() {
    let settings = chromata::popular::gruvbox::DARK_HARD.to_syntect_settings();
    assert!(settings.foreground.is_some());
    assert!(settings.background.is_some());
}

#[cfg(feature = "colored-integration")]
#[test]
fn colored_to_colored_string() {
    let _styled = chromata::popular::gruvbox::DARK_HARD.to_colored_string("test");
}

#[cfg(feature = "cursive-integration")]
#[test]
fn cursive_palette() {
    let mut palette = cursive_core::theme::Palette::default();
    chromata::popular::gruvbox::DARK_HARD.apply_to_cursive_palette(&mut palette);
}

#[cfg(feature = "comfy-table-integration")]
#[test]
fn comfy_table_style() {
    let cell = comfy_table::Cell::new("test");
    let _styled = chromata::popular::gruvbox::DARK_HARD.to_comfy_table_cell(cell);
}

#[cfg(feature = "ratatui-integration")]
#[test]
fn ratatui_style() {
    let style = chromata::popular::gruvbox::DARK_HARD.to_ratatui_style();
    // Verify style has fg and bg set (non-default)
    assert_ne!(style, ratatui::style::Style::default());
}

#[cfg(feature = "egui-integration")]
#[test]
fn egui_apply_visuals() {
    let mut visuals = egui::Visuals::default();
    chromata::popular::gruvbox::DARK_HARD.apply_to_egui_visuals(&mut visuals);
    assert!(visuals.dark_mode);
}

// --- Boundary value tests ---

#[cfg(feature = "bevy-color-integration")]
#[test]
fn bevy_color_boundary_black() {
    let c = chromata::Color::new(0, 0, 0);
    let p: bevy_color::Srgba = c.into();
    assert_eq!(p.red, 0.0);
    assert_eq!(p.green, 0.0);
    assert_eq!(p.blue, 0.0);
}

#[cfg(feature = "bevy-color-integration")]
#[test]
fn bevy_color_boundary_white() {
    let c = chromata::Color::new(255, 255, 255);
    let p: bevy_color::Srgba = c.into();
    assert_eq!(p.red, 1.0);
    assert_eq!(p.green, 1.0);
    assert_eq!(p.blue, 1.0);
}

#[cfg(feature = "wgpu-integration")]
#[test]
fn wgpu_boundary_black() {
    let c = chromata::Color::new(0, 0, 0);
    let p: wgpu::Color = c.into();
    assert_eq!(p.r, 0.0);
    assert_eq!(p.g, 0.0);
    assert_eq!(p.b, 0.0);
}

#[cfg(feature = "wgpu-integration")]
#[test]
fn wgpu_boundary_white() {
    let c = chromata::Color::new(255, 255, 255);
    let p: wgpu::Color = c.into();
    assert_eq!(p.r, 1.0);
    assert_eq!(p.g, 1.0);
    assert_eq!(p.b, 1.0);
}

#[cfg(feature = "iced-integration")]
#[test]
fn iced_boundary_black() {
    let c = chromata::Color::new(0, 0, 0);
    let p: iced_core::Color = c.into();
    assert_eq!(p.r, 0.0);
    assert_eq!(p.g, 0.0);
    assert_eq!(p.b, 0.0);
}

#[cfg(feature = "iced-integration")]
#[test]
fn iced_boundary_white() {
    let c = chromata::Color::new(255, 255, 255);
    let p: iced_core::Color = c.into();
    assert_eq!(p.r, 1.0);
    assert_eq!(p.g, 1.0);
    assert_eq!(p.b, 1.0);
}

#[cfg(feature = "macroquad-integration")]
#[test]
fn macroquad_boundary_black() {
    let c = chromata::Color::new(0, 0, 0);
    let p: macroquad::color::Color = c.into();
    assert_eq!(p.r, 0.0);
    assert_eq!(p.g, 0.0);
    assert_eq!(p.b, 0.0);
}

#[cfg(feature = "macroquad-integration")]
#[test]
fn macroquad_boundary_white() {
    let c = chromata::Color::new(255, 255, 255);
    let p: macroquad::color::Color = c.into();
    assert_eq!(p.r, 1.0);
    assert_eq!(p.g, 1.0);
    assert_eq!(p.b, 1.0);
}
