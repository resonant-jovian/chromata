use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Base16Scheme {
    #[allow(dead_code)]
    system: Option<String>,
    name: String,
    author: String,
    variant: Option<String>,
    palette: BTreeMap<String, String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("fetch") => fetch(),
        Some("generate") => generate(),
        Some(other) => {
            eprintln!("Unknown command: {other}");
            eprintln!("Usage: cargo xtask [fetch|generate]");
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: cargo xtask [fetch|generate]");
            std::process::exit(1);
        }
    }
}

fn fetch() {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("base16");
    fs::create_dir_all(&data_dir).expect("failed to create data/base16/");

    let tmp_dir = project_root.join(".xtask-tmp-schemes");

    // Clean up any previous failed fetch
    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).expect("failed to clean up old temp dir");
    }

    println!("Cloning tinted-theming/schemes...");
    let status = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/tinted-theming/schemes.git",
            tmp_dir.to_str().unwrap(),
        ])
        .status()
        .expect("failed to run git clone — is git installed?");

    if !status.success() {
        eprintln!("git clone failed with exit code: {}", status);
        std::process::exit(1);
    }

    let schemes_dir = tmp_dir.join("base16");
    if !schemes_dir.exists() {
        eprintln!("No base16/ directory found in cloned schemes repo");
        fs::remove_dir_all(&tmp_dir).ok();
        std::process::exit(1);
    }

    let mut count = 0;
    for entry in fs::read_dir(&schemes_dir).expect("failed to read schemes/base16/") {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();

        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("yaml") && ext != Some("yml") {
            continue;
        }

        let file_name = path.file_name().unwrap();
        let dest = data_dir.join(file_name);
        fs::copy(&path, &dest).unwrap_or_else(|e| panic!("failed to copy {}: {e}", path.display()));
        count += 1;
    }

    // Clean up temp dir
    fs::remove_dir_all(&tmp_dir).expect("failed to clean up temp dir");

    println!("Fetched {count} base16 schemes to data/base16/");
}

fn generate() {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("base16");
    let output_dir = project_root.join("src").join("base16");

    if !data_dir.exists() {
        eprintln!(
            "No data/base16/ directory found at {}.\nRun `cargo xtask fetch` first.",
            data_dir.display()
        );
        std::process::exit(1);
    }

    let mut generated_modules: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(&data_dir).expect("failed to read data/base16/") {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();

        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("yaml") && ext != Some("yml") {
            continue;
        }

        let content = fs::read_to_string(&path).expect("failed to read YAML file");
        let scheme: Base16Scheme = match serde_yaml::from_str(&content) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  Skipping {}: {e}", path.display());
                continue;
            }
        };

        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let module_name = sanitize_module_name(file_stem);
        let const_name = sanitize_const_name(&scheme.name);
        let variant = scheme.variant.as_deref().unwrap_or("dark");

        let rust_code = generate_theme_const(&scheme, variant);
        let output_path = output_dir.join(format!("{module_name}.rs"));

        fs::write(&output_path, rust_code)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", output_path.display()));

        println!("  Generated: src/base16/{module_name}.rs ({const_name})");
        generated_modules.push((module_name, const_name));
    }

    // Sort deterministically by module name
    generated_modules.sort_by(|a, b| a.0.cmp(&b.0));

    // Update src/base16/mod.rs
    let mod_content = generate_mod_rs(&generated_modules);
    let mod_path = output_dir.join("mod.rs");
    fs::write(&mod_path, mod_content)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", mod_path.display()));

    println!(
        "\nGenerated {} base16 theme(s). Updated src/base16/mod.rs.",
        generated_modules.len()
    );
}

fn generate_theme_const(scheme: &Base16Scheme, variant: &str) -> String {
    let get_hex = |key: &str| -> String {
        let hex = scheme
            .palette
            .get(key)
            .or_else(|| scheme.palette.get(&key.to_uppercase()))
            .or_else(|| {
                let lower = key.to_lowercase();
                scheme.palette.get(&lower)
            })
            .unwrap_or_else(|| panic!("missing palette key {key} in scheme {}", scheme.name));
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        format!("0x{}", hex.to_lowercase())
    };

    let get_hex_u32 = |key: &str| -> u32 {
        let hex = scheme
            .palette
            .get(key)
            .or_else(|| scheme.palette.get(&key.to_uppercase()))
            .or_else(|| {
                let lower = key.to_lowercase();
                scheme.palette.get(&lower)
            })
            .unwrap_or_else(|| panic!("missing palette key {key} in scheme {}", scheme.name));
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        u32::from_str_radix(hex, 16).unwrap_or_else(|e| {
            panic!(
                "invalid hex '{hex}' for {key} in scheme {}: {e}",
                scheme.name
            )
        })
    };

    let variant_enum = match variant {
        "light" => "Light",
        _ => "Dark",
    };

    let bg_hex = get_hex_u32("base00");
    let fg_hex = get_hex_u32("base05");
    let contrast_enum = classify_contrast(bg_hex, fg_hex);

    let name = &scheme.name;
    let author = scheme.author.replace('"', "\\\"");

    let base00 = get_hex("base00");
    let base01 = get_hex("base01");
    let base02 = get_hex("base02");
    let base03 = get_hex("base03");
    let base04 = get_hex("base04");
    let base05 = get_hex("base05");
    let _base06 = get_hex("base06");
    let _base07 = get_hex("base07");
    let base08 = get_hex("base08");
    let base09 = get_hex("base09");
    let base0a = get_hex("base0A");
    let base0b = get_hex("base0B");
    let base0c = get_hex("base0C");
    let base0d = get_hex("base0D");
    let base0e = get_hex("base0E");
    let base0f = get_hex("base0F");

    let escaped_name = name.replace('"', "\\\"");

    format!(
        r#"//! {name} color theme.
//!
//! Auto-generated by `cargo xtask generate` — do not edit.

use crate::{{Color, Contrast, Theme, Variant}};

/// {name}
///
/// Author: {author}
/// Variant: {variant_enum}
/// Contrast: {contrast_enum}
/// Source: base16 (tinted-theming/schemes)
pub const THEME: Theme = Theme {{
    name: "{escaped_name}",
    author: "{author}",
    variant: Variant::{variant_enum},
    contrast: Contrast::{contrast_enum},
    bg: Color::from_hex({base00}),
    fg: Color::from_hex({base05}),
    cursor: Some(Color::from_hex({base05})),
    selection: Some(Color::from_hex({base02})),
    line_highlight: Some(Color::from_hex({base01})),
    gutter: Some(Color::from_hex({base03})),
    statusbar_bg: Some(Color::from_hex({base01})),
    statusbar_fg: Some(Color::from_hex({base04})),
    comment: Some(Color::from_hex({base03})),
    keyword: Some(Color::from_hex({base0e})),
    string: Some(Color::from_hex({base0b})),
    function: Some(Color::from_hex({base0d})),
    variable: Some(Color::from_hex({base08})),
    r#type: Some(Color::from_hex({base0a})),
    constant: Some(Color::from_hex({base09})),
    operator: Some(Color::from_hex({base05})),
    tag: Some(Color::from_hex({base08})),
    error: Some(Color::from_hex({base08})),
    warning: Some(Color::from_hex({base0a})),
    info: Some(Color::from_hex({base0d})),
    success: Some(Color::from_hex({base0b})),
    red: Some(Color::from_hex({base08})),
    orange: Some(Color::from_hex({base09})),
    yellow: Some(Color::from_hex({base0a})),
    green: Some(Color::from_hex({base0b})),
    cyan: Some(Color::from_hex({base0c})),
    blue: Some(Color::from_hex({base0d})),
    purple: Some(Color::from_hex({base0e})),
    magenta: Some(Color::from_hex({base0f})),
}};
"#
    )
}

fn generate_mod_rs(modules: &[(String, String)]) -> String {
    let mut output = String::from(
        "//! Base16 color themes.\n\
         //!\n\
         //! Themes sourced from [tinted-theming/schemes](https://github.com/tinted-theming/schemes).\n\
         //! Each theme provides the standard 16 base16 palette slots mapped to\n\
         //! semantic color roles.\n\
         //!\n\
         //! Enable with the `base16` feature flag.\n\
         //!\n\
         //! Auto-generated by `cargo xtask generate` — do not edit.\n\n\
         use crate::Theme;\n\n",
    );

    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {module_name};\n"));
    }

    output.push_str("\n/// All themes in the `base16` collection.\n");
    output.push_str("pub static THEMES: &[&Theme] = &[\n");
    for (module_name, _) in modules {
        output.push_str(&format!("    &{module_name}::THEME,\n"));
    }
    output.push_str("];\n");

    output
}

fn sanitize_module_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    // Collapse consecutive underscores and trim
    let mut result = String::new();
    let mut prev_underscore = true; // trim leading
    for c in sanitized.chars() {
        if c == '_' {
            if !prev_underscore {
                result.push('_');
            }
            prev_underscore = true;
        } else {
            result.push(c);
            prev_underscore = false;
        }
    }
    let result = result.trim_matches('_').to_string();
    // Rust identifiers cannot start with a digit
    if result.starts_with(|c: char| c.is_ascii_digit()) {
        format!("_{result}")
    } else {
        result
    }
}

fn sanitize_const_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

fn project_root() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.pop(); // Go up from xtask/ to project root
    path
}

// --- WCAG contrast detection ---

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn luminance(hex: u32) -> f64 {
    let r = srgb_to_linear(((hex >> 16) & 0xFF) as f64 / 255.0);
    let g = srgb_to_linear(((hex >> 8) & 0xFF) as f64 / 255.0);
    let b = srgb_to_linear((hex & 0xFF) as f64 / 255.0);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn contrast_ratio(hex1: u32, hex2: u32) -> f64 {
    let l1 = luminance(hex1);
    let l2 = luminance(hex2);
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

fn classify_contrast(bg_hex: u32, fg_hex: u32) -> &'static str {
    let ratio = contrast_ratio(bg_hex, fg_hex);
    if ratio >= 10.0 {
        "High"
    } else if ratio >= 4.5 {
        "Normal"
    } else {
        "Low"
    }
}
