use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

use crate::codegen::{
    classify_contrast, detect_variant, project_root, sanitize_const_name, sanitize_module_name,
    write_if_changed,
};

/// Unified representation after normalizing both YAML formats.
struct NormalizedBase24 {
    name: String,
    author: String,
    variant: String,
    palette: BTreeMap<String, String>,
}

/// Modern format: `system: "base24"` + nested `palette:` block.
#[derive(Debug, Deserialize)]
struct ModernBase24 {
    #[allow(dead_code)]
    system: Option<String>,
    name: String,
    author: String,
    variant: Option<String>,
    palette: BTreeMap<String, String>,
}

/// Legacy flat format: top-level base00 through base17 keys.
#[derive(Debug, Deserialize)]
struct LegacyBase24 {
    scheme: Option<String>,
    name: Option<String>,
    author: String,
    #[serde(flatten)]
    colors: BTreeMap<String, String>,
}

const BASE24_KEYS: &[&str] = &[
    "base00", "base01", "base02", "base03", "base04", "base05", "base06", "base07", "base08",
    "base09", "base0A", "base0B", "base0C", "base0D", "base0E", "base0F", "base10", "base11",
    "base12", "base13", "base14", "base15", "base16", "base17",
];

fn normalize(content: &str, file_stem: &str) -> Result<Option<NormalizedBase24>> {
    // Try modern format first
    if let Ok(modern) = serde_yaml::from_str::<ModernBase24>(content) {
        if modern.palette.keys().any(|k| k.starts_with("base")) {
            return Ok(Some(NormalizedBase24 {
                name: modern.name,
                author: modern.author,
                variant: modern.variant.unwrap_or_else(|| "dark".into()),
                palette: modern.palette,
            }));
        }
    }

    // Try legacy flat format
    if let Ok(legacy) = serde_yaml::from_str::<LegacyBase24>(content) {
        if legacy.colors.keys().any(|k| k.starts_with("base")) {
            let name = legacy
                .name
                .or(legacy.scheme)
                .unwrap_or_else(|| file_stem.replace('-', " ").replace('_', " "));
            return Ok(Some(NormalizedBase24 {
                name,
                author: legacy.author,
                variant: "dark".into(),
                palette: legacy.colors,
            }));
        }
    }

    Ok(None)
}

fn get_hex_literal(
    palette: &BTreeMap<String, String>,
    key: &str,
    scheme_name: &str,
) -> Result<String> {
    let hex = palette
        .get(key)
        .or_else(|| palette.get(&key.to_uppercase()))
        .or_else(|| palette.get(&key.to_lowercase()))
        .with_context(|| format!("missing palette key {key} in scheme {scheme_name}"))?;
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    Ok(format!("0x{}", hex.to_lowercase()))
}

fn get_hex_u32(palette: &BTreeMap<String, String>, key: &str, scheme_name: &str) -> Result<u32> {
    let hex = palette
        .get(key)
        .or_else(|| palette.get(&key.to_uppercase()))
        .or_else(|| palette.get(&key.to_lowercase()))
        .with_context(|| format!("missing palette key {key} in scheme {scheme_name}"))?;
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    u32::from_str_radix(hex, 16)
        .with_context(|| format!("invalid hex '{hex}' for {key} in scheme {scheme_name}"))
}

pub fn generate() -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("base24");
    let output_dir = project_root.join("src").join("base24");

    if !data_dir.exists() {
        anyhow::bail!("No data/base24/ directory found.\nRun `cargo xtask fetch base24` first.");
    }

    fs::create_dir_all(&output_dir).context("creating src/base24/")?;

    let mut generated_modules: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(&data_dir).context("reading data/base24/")? {
        let entry = entry?;
        let path = entry.path();

        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("yaml") && ext != Some("yml") {
            continue;
        }

        let content =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;

        let file_stem = path.file_stem().unwrap().to_str().unwrap();

        let scheme = match normalize(&content, file_stem)? {
            Some(s) => s,
            None => {
                eprintln!("  Skipping {}: could not parse as base24", path.display());
                continue;
            }
        };

        // Validate all 24 base keys are present
        let mut missing = Vec::new();
        for key in BASE24_KEYS {
            if scheme.palette.get(*key).is_none()
                && scheme.palette.get(&key.to_uppercase()).is_none()
                && scheme.palette.get(&key.to_lowercase()).is_none()
            {
                missing.push(*key);
            }
        }

        if !missing.is_empty() {
            eprintln!(
                "  Skipping {}: missing keys: {}",
                path.display(),
                missing.join(", ")
            );
            continue;
        }

        let module_name = sanitize_module_name(file_stem);
        let const_name = sanitize_const_name(&scheme.name);

        let rust_code = generate_theme_const(&scheme)?;
        let output_path = output_dir.join(format!("{module_name}.rs"));

        if write_if_changed(&output_path, &rust_code)? {
            println!("  Generated: src/base24/{module_name}.rs ({const_name})");
        }

        generated_modules.push((module_name, const_name));
    }

    generated_modules.sort_by(|a, b| a.0.cmp(&b.0));

    let mod_content = generate_mod_rs(&generated_modules);
    let mod_path = output_dir.join("mod.rs");
    write_if_changed(&mod_path, &mod_content)?;

    println!(
        "\nGenerated {} base24 theme(s). Updated src/base24/mod.rs.",
        generated_modules.len()
    );
    Ok(())
}

fn generate_theme_const(scheme: &NormalizedBase24) -> Result<String> {
    let bg_hex = get_hex_u32(&scheme.palette, "base00", &scheme.name)?;
    let fg_hex = get_hex_u32(&scheme.palette, "base05", &scheme.name)?;
    let contrast_enum = classify_contrast(bg_hex, fg_hex);
    // Use luminance-based detection rather than trusting YAML metadata,
    // since some upstream schemes have incorrect variant labels.
    let variant_enum = detect_variant(bg_hex);

    let name = &scheme.name;
    let author = scheme.author.replace('"', "\\\"");
    let escaped_name = name.replace('"', "\\\"");

    let p = &scheme.palette;
    let sn = &scheme.name;

    let base00 = get_hex_literal(p, "base00", sn)?;
    let base01 = get_hex_literal(p, "base01", sn)?;
    let base02 = get_hex_literal(p, "base02", sn)?;
    let base03 = get_hex_literal(p, "base03", sn)?;
    let base04 = get_hex_literal(p, "base04", sn)?;
    let base05 = get_hex_literal(p, "base05", sn)?;
    let base06 = get_hex_literal(p, "base06", sn)?;
    let base07 = get_hex_literal(p, "base07", sn)?;
    let base08 = get_hex_literal(p, "base08", sn)?;
    let base09 = get_hex_literal(p, "base09", sn)?;
    let base0a = get_hex_literal(p, "base0A", sn)?;
    let base0b = get_hex_literal(p, "base0B", sn)?;
    let base0c = get_hex_literal(p, "base0C", sn)?;
    let base0d = get_hex_literal(p, "base0D", sn)?;
    let base0e = get_hex_literal(p, "base0E", sn)?;
    let base0f = get_hex_literal(p, "base0F", sn)?;
    let base10 = get_hex_literal(p, "base10", sn)?;
    let base11 = get_hex_literal(p, "base11", sn)?;
    let base12 = get_hex_literal(p, "base12", sn)?;
    let base13 = get_hex_literal(p, "base13", sn)?;
    let base14 = get_hex_literal(p, "base14", sn)?;
    let base15 = get_hex_literal(p, "base15", sn)?;
    let base16 = get_hex_literal(p, "base16", sn)?;
    let base17 = get_hex_literal(p, "base17", sn)?;

    Ok(format!(
        r#"//! {name} color theme (base24).
//!
//! Auto-generated by `cargo xtask generate base24` — do not edit.

use crate::{{Base16Palette, Base24Palette, Color, Contrast, Theme, Variant}};

/// {name}
///
/// Author: {author}
/// Variant: {variant_enum}
/// Contrast: {contrast_enum}
/// Source: base24 (tinted-theming/schemes)
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
    red: Some(Color::from_hex({base12})),
    orange: Some(Color::from_hex({base09})),
    yellow: Some(Color::from_hex({base13})),
    green: Some(Color::from_hex({base14})),
    cyan: Some(Color::from_hex({base0c})),
    blue: Some(Color::from_hex({base15})),
    purple: Some(Color::from_hex({base16})),
    magenta: Some(Color::from_hex({base17})),
}};

/// Full base24 palette with all 24 color slots.
pub const PALETTE: Base24Palette = Base24Palette {{
    base: Base16Palette {{
        base00: Color::from_hex({base00}),
        base01: Color::from_hex({base01}),
        base02: Color::from_hex({base02}),
        base03: Color::from_hex({base03}),
        base04: Color::from_hex({base04}),
        base05: Color::from_hex({base05}),
        base06: Color::from_hex({base06}),
        base07: Color::from_hex({base07}),
        base08: Color::from_hex({base08}),
        base09: Color::from_hex({base09}),
        base0a: Color::from_hex({base0a}),
        base0b: Color::from_hex({base0b}),
        base0c: Color::from_hex({base0c}),
        base0d: Color::from_hex({base0d}),
        base0e: Color::from_hex({base0e}),
        base0f: Color::from_hex({base0f}),
    }},
    base10: Color::from_hex({base10}),
    base11: Color::from_hex({base11}),
    base12: Color::from_hex({base12}),
    base13: Color::from_hex({base13}),
    base14: Color::from_hex({base14}),
    base15: Color::from_hex({base15}),
    base16: Color::from_hex({base16}),
    base17: Color::from_hex({base17}),
}};
"#
    ))
}

fn generate_mod_rs(modules: &[(String, String)]) -> String {
    let mut output = String::from(
        "//! Base24 color themes.\n\
         //!\n\
         //! Themes sourced from [tinted-theming/schemes](https://github.com/tinted-theming/schemes).\n\
         //! Each theme provides the standard 24 base24 palette slots (16 base16 +\n\
         //! 8 extended accent colors) mapped to semantic color roles.\n\
         //!\n\
         //! Enable with the `base24` feature flag.\n\
         //!\n\
         //! Auto-generated by `cargo xtask generate base24` — do not edit.\n\n\
         use crate::Theme;\n\n",
    );

    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {module_name};\n"));
    }

    output.push_str("\n/// All themes in the `base24` collection.\n");
    output.push_str("pub static THEMES: &[&Theme] = &[\n");
    for (module_name, _) in modules {
        output.push_str(&format!("    &{module_name}::THEME,\n"));
    }
    output.push_str("];\n");

    output
}
