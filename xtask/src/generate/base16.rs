use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;

use crate::codegen::{
    classify_contrast, project_root, sanitize_const_name, sanitize_module_name, write_if_changed,
};

#[derive(Debug, Deserialize)]
pub struct Base16Scheme {
    #[allow(dead_code)]
    pub system: Option<String>,
    pub name: String,
    pub author: String,
    pub variant: Option<String>,
    pub palette: BTreeMap<String, String>,
}

impl Base16Scheme {
    /// Get a hex color string for a palette key, trying multiple case variants.
    pub fn get_hex(&self, key: &str) -> Option<&str> {
        self.palette
            .get(key)
            .or_else(|| self.palette.get(&key.to_uppercase()))
            .or_else(|| self.palette.get(&key.to_lowercase()))
            .map(|s| s.as_str())
    }

    /// Get hex string formatted as `0xaabbcc` for code generation.
    pub fn get_hex_literal(&self, key: &str) -> Result<String> {
        let hex = self
            .get_hex(key)
            .with_context(|| format!("missing palette key {key} in scheme {}", self.name))?;
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        Ok(format!("0x{}", hex.to_lowercase()))
    }

    /// Get hex value as u32.
    pub fn get_hex_u32(&self, key: &str) -> Result<u32> {
        let hex = self
            .get_hex(key)
            .with_context(|| format!("missing palette key {key} in scheme {}", self.name))?;
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        u32::from_str_radix(hex, 16)
            .with_context(|| format!("invalid hex '{hex}' for {key} in scheme {}", self.name))
    }
}

pub fn generate() -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("base16");
    let output_dir = project_root.join("src").join("base16");

    if !data_dir.exists() {
        anyhow::bail!(
            "No data/base16/ directory found at {}.\nRun `cargo xtask fetch base16` first.",
            data_dir.display()
        );
    }

    let mut generated_modules: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(&data_dir).context("reading data/base16/")? {
        let entry = entry.context("reading directory entry")?;
        let path = entry.path();

        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("yaml") && ext != Some("yml") {
            continue;
        }

        let content =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
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

        let rust_code = generate_theme_const(&scheme, variant)?;
        let output_path = output_dir.join(format!("{module_name}.rs"));

        if write_if_changed(&output_path, &rust_code)? {
            println!("  Generated: src/base16/{module_name}.rs ({const_name})");
        }

        generated_modules.push((module_name, const_name));
    }

    generated_modules.sort_by(|a, b| a.0.cmp(&b.0));

    let mod_content = generate_mod_rs(&generated_modules);
    let mod_path = output_dir.join("mod.rs");
    write_if_changed(&mod_path, &mod_content)?;

    println!(
        "\nGenerated {} base16 theme(s). Updated src/base16/mod.rs.",
        generated_modules.len()
    );
    Ok(())
}

fn generate_theme_const(scheme: &Base16Scheme, variant: &str) -> Result<String> {
    let variant_enum = match variant {
        "light" => "Light",
        _ => "Dark",
    };

    let bg_hex = scheme.get_hex_u32("base00")?;
    let fg_hex = scheme.get_hex_u32("base05")?;
    let contrast_enum = classify_contrast(bg_hex, fg_hex);

    let name = &scheme.name;
    let author = scheme.author.replace('"', "\\\"");
    let author_doc = if author.is_empty() {
        "/// Author:".to_string()
    } else {
        format!("/// Author: {author}")
    };
    let escaped_name = name.replace('"', "\\\"");

    let base00 = scheme.get_hex_literal("base00")?;
    let base01 = scheme.get_hex_literal("base01")?;
    let base02 = scheme.get_hex_literal("base02")?;
    let base03 = scheme.get_hex_literal("base03")?;
    let base04 = scheme.get_hex_literal("base04")?;
    let base05 = scheme.get_hex_literal("base05")?;
    let _base06 = scheme.get_hex_literal("base06")?;
    let _base07 = scheme.get_hex_literal("base07")?;
    let base08 = scheme.get_hex_literal("base08")?;
    let base09 = scheme.get_hex_literal("base09")?;
    let base0a = scheme.get_hex_literal("base0A")?;
    let base0b = scheme.get_hex_literal("base0B")?;
    let base0c = scheme.get_hex_literal("base0C")?;
    let base0d = scheme.get_hex_literal("base0D")?;
    let base0e = scheme.get_hex_literal("base0E")?;
    let base0f = scheme.get_hex_literal("base0F")?;

    Ok(format!(
        r#"//! {name} color theme.
//!
//! Auto-generated by `cargo xtask generate base16` — do not edit.

use crate::{{Color, Contrast, Theme, Variant}};

/// {name}
///
{author_doc}
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
    ))
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
         //! Auto-generated by `cargo xtask generate base16` — do not edit.\n\n\
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
