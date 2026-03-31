use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

use crate::codegen::{
    classify_contrast, detect_variant, project_root, sanitize_const_name, sanitize_module_name,
    write_if_changed,
};

// ── xterm-256 palette ──────────────────────────────────────────────

/// Standard ANSI colors (indices 0-15).
static ANSI_COLORS: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // 0 Black
    (128, 0, 0),     // 1 Red
    (0, 128, 0),     // 2 Green
    (128, 128, 0),   // 3 Yellow
    (0, 0, 128),     // 4 Blue
    (128, 0, 128),   // 5 Magenta
    (0, 128, 128),   // 6 Cyan
    (192, 192, 192), // 7 White
    (128, 128, 128), // 8 Bright Black
    (255, 0, 0),     // 9 Bright Red
    (0, 255, 0),     // 10 Bright Green
    (255, 255, 0),   // 11 Bright Yellow
    (0, 0, 255),     // 12 Bright Blue
    (255, 0, 255),   // 13 Bright Magenta
    (0, 255, 255),   // 14 Bright Cyan
    (255, 255, 255), // 15 Bright White
];

fn xterm256_to_rgb(idx: u8) -> (u8, u8, u8) {
    if idx < 16 {
        return ANSI_COLORS[idx as usize];
    }
    if idx >= 232 {
        // Grayscale ramp: 232-255 -> 8, 18, 28, ..., 238
        let v = 8 + 10 * (idx - 232);
        return (v, v, v);
    }
    // 6x6x6 color cube: indices 16-231
    let idx = idx - 16;
    let r_idx = idx / 36;
    let g_idx = (idx % 36) / 6;
    let b_idx = idx % 6;
    let to_val = |i: u8| if i == 0 { 0 } else { 55 + 40 * i };
    (to_val(r_idx), to_val(g_idx), to_val(b_idx))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

// ── Pre-filter ─────────────────────────────────────────────────────

fn should_skip(content: &str, file_size: u64) -> bool {
    if file_size > 50_000 {
        return true;
    }
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip files with complex scripting
        if trimmed.starts_with("function!") || trimmed.starts_with("fun!") {
            return true;
        }
        // Skip files using execute with string building
        if trimmed.contains("execute") && (trimmed.contains('"') || trimmed.contains('.')) {
            // But allow simple `execute "set background=..."` patterns
            if !trimmed.contains("set background") {
                return true;
            }
        }
        // Skip files that source other files
        if (trimmed.starts_with("source ") || trimmed.starts_with("runtime "))
            && !trimmed.starts_with("source.")
        {
            return true;
        }
    }
    false
}

// ── Highlight group data ───────────────────────────────────────────

#[derive(Debug, Default, Clone)]
struct HighlightGroup {
    guifg: Option<u32>,
    guibg: Option<u32>,
}

// ── 3-pass parser ──────────────────────────────────────────────────

fn parse_vim_colorscheme(content: &str) -> Option<VimThemeData> {
    // Pass 1: extract variables
    let var_re = Regex::new(r#"let\s+(?:s:|g:)?(\w+)\s*=\s*['"]#?([0-9a-fA-F]{6})['"]"#).unwrap();
    let mut vars: HashMap<String, u32> = HashMap::new();
    for cap in var_re.captures_iter(content) {
        let name = cap[1].to_string();
        if let Ok(hex) = u32::from_str_radix(&cap[2], 16) {
            vars.insert(name, hex);
        }
    }

    // Pass 2: extract highlight groups
    let hi_re = Regex::new(r"(?i)hi(?:ghlight)?!?\s+(\w+)\s+(.+)").unwrap();
    let link_re =
        Regex::new(r"(?i)hi(?:ghlight)?!?\s+(?:def(?:ault)?\s+)?link\s+(\w+)\s+(\w+)").unwrap();
    let prop_re = Regex::new(r"(guifg|guibg|ctermfg|ctermbg)\s*=\s*(\S+)").unwrap();

    let mut groups: HashMap<String, HighlightGroup> = HashMap::new();
    let mut links: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('"') || trimmed.is_empty() {
            continue;
        }

        // Check for link first
        if let Some(cap) = link_re.captures(trimmed) {
            links.push((cap[1].to_string(), cap[2].to_string()));
            continue;
        }

        // Check for highlight definition
        if let Some(cap) = hi_re.captures(trimmed) {
            let group_name = cap[1].to_string();
            if group_name.to_lowercase() == "link" || group_name.to_lowercase() == "clear" {
                continue;
            }
            let props_str = &cap[2];

            let mut group = groups.entry(group_name).or_default().clone();

            for prop_cap in prop_re.captures_iter(props_str) {
                let key = prop_cap[1].to_lowercase();
                let value = prop_cap[2].to_string();

                let resolved = resolve_color_value(&value, &vars);

                match key.as_str() {
                    "guifg" => {
                        if let Some(hex) = resolved {
                            group.guifg = Some(hex);
                        }
                    }
                    "guibg" => {
                        if let Some(hex) = resolved {
                            group.guibg = Some(hex);
                        }
                    }
                    "ctermfg" => {
                        if group.guifg.is_none()
                            && let Some(hex) = resolved
                        {
                            group.guifg = Some(hex);
                        }
                    }
                    "ctermbg" => {
                        if group.guibg.is_none()
                            && let Some(hex) = resolved
                        {
                            group.guibg = Some(hex);
                        }
                    }
                    _ => {}
                }
            }

            groups.insert(cap[1].to_string(), group);
        }
    }

    // Pass 3: resolve links
    for (from, to) in &links {
        let resolved = resolve_link(to, &groups, &links, 0);
        if let Some(group) = resolved {
            let entry = groups.entry(from.clone()).or_default();
            if entry.guifg.is_none() {
                entry.guifg = group.guifg;
            }
            if entry.guibg.is_none() {
                entry.guibg = group.guibg;
            }
        }
    }

    // Extract theme name
    let name_re = Regex::new(r#"let\s+(?:g:)?colors_name\s*=\s*['"](.*?)['"]"#).unwrap();
    let theme_name = name_re.captures(content).map(|c| c[1].to_string());

    // Extract variant
    let bg_re = Regex::new(r"set\s+background\s*=\s*(\w+)").unwrap();
    let variant_str = bg_re.captures(content).map(|c| c[1].to_string());

    // Map to theme
    let normal = groups.get("Normal")?;
    let bg = normal.guibg?;
    let fg = normal.guifg?;

    if bg == fg {
        return None;
    }

    let get_fg = |names: &[&str]| -> Option<u32> {
        for name in names {
            if let Some(g) = groups.get(*name)
                && let Some(c) = g.guifg
            {
                return Some(c);
            }
        }
        None
    };

    let get_bg_of = |names: &[&str]| -> Option<u32> {
        for name in names {
            if let Some(g) = groups.get(*name)
                && let Some(c) = g.guibg
            {
                return Some(c);
            }
        }
        None
    };

    let comment = get_fg(&["Comment"]);
    let keyword = get_fg(&["Keyword", "Statement"]);
    let string = get_fg(&["String"]);
    let function = get_fg(&["Function"]);
    let variable = get_fg(&["Identifier"]);
    let type_color = get_fg(&["Type"]);
    let constant = get_fg(&["Constant"]);

    // Quality gate: bg + fg + 4 syntax colors
    let syntax_count = [
        comment, keyword, string, function, variable, type_color, constant,
    ]
    .iter()
    .filter(|c| c.is_some())
    .count();
    if syntax_count < 4 {
        return None;
    }

    let error = get_fg(&["ErrorMsg", "Error"]);
    let warning = get_fg(&["WarningMsg"]);

    // Derive accent colors
    let diff_add_fg = get_fg(&["DiffAdd"]);
    let diff_delete_fg = get_fg(&["DiffDelete"]);

    Some(VimThemeData {
        name: theme_name,
        _variant: variant_str,
        bg,
        fg,
        cursor: get_bg_of(&["Cursor"]).or_else(|| get_fg(&["Cursor"])),
        selection: get_bg_of(&["Visual"]),
        line_highlight: get_bg_of(&["CursorLine"]),
        gutter: get_fg(&["LineNr"]),
        statusbar_bg: get_bg_of(&["StatusLine"]),
        statusbar_fg: get_fg(&["StatusLine"]),
        comment,
        keyword,
        string,
        function,
        variable,
        r#type: type_color,
        constant,
        operator: get_fg(&["Operator"]),
        tag: get_fg(&["Tag"]),
        error,
        warning,
        red: error.or(diff_delete_fg),
        orange: None,
        yellow: warning.or_else(|| get_fg(&["Special"])),
        green: string.or(diff_add_fg),
        cyan: type_color,
        blue: function,
        purple: keyword,
        magenta: get_fg(&["PreProc"]),
    })
}

fn resolve_color_value(value: &str, vars: &HashMap<String, u32>) -> Option<u32> {
    let value = value.trim_matches(|c: char| c == '\'' || c == '"');
    let lower = value.to_lowercase();

    if lower == "none" || lower == "fg" || lower == "bg" {
        return None;
    }

    // #RRGGBB hex
    let stripped = value.strip_prefix('#').unwrap_or(value);
    if stripped.len() == 6
        && let Ok(hex) = u32::from_str_radix(stripped, 16)
    {
        return Some(hex);
    }

    // Variable reference
    let var_name = value
        .strip_prefix("s:")
        .or_else(|| value.strip_prefix("g:"))
        .unwrap_or(value);
    if let Some(&hex) = vars.get(var_name) {
        return Some(hex);
    }

    // cterm index (decimal number)
    if let Ok(idx) = value.parse::<u16>()
        && idx <= 255
    {
        let (r, g, b) = xterm256_to_rgb(idx as u8);
        return Some(rgb_to_hex(r, g, b));
    }

    None
}

fn resolve_link(
    target: &str,
    groups: &HashMap<String, HighlightGroup>,
    links: &[(String, String)],
    depth: u8,
) -> Option<HighlightGroup> {
    if depth > 10 {
        return None;
    }
    if let Some(group) = groups.get(target)
        && (group.guifg.is_some() || group.guibg.is_some())
    {
        return Some(group.clone());
    }
    // Follow link chain
    for (from, to) in links {
        if from == target {
            return resolve_link(to, groups, links, depth + 1);
        }
    }
    groups.get(target).cloned()
}

struct VimThemeData {
    name: Option<String>,
    _variant: Option<String>,
    bg: u32,
    fg: u32,
    cursor: Option<u32>,
    selection: Option<u32>,
    line_highlight: Option<u32>,
    gutter: Option<u32>,
    statusbar_bg: Option<u32>,
    statusbar_fg: Option<u32>,
    comment: Option<u32>,
    keyword: Option<u32>,
    string: Option<u32>,
    function: Option<u32>,
    variable: Option<u32>,
    r#type: Option<u32>,
    constant: Option<u32>,
    operator: Option<u32>,
    tag: Option<u32>,
    error: Option<u32>,
    warning: Option<u32>,
    red: Option<u32>,
    orange: Option<u32>,
    yellow: Option<u32>,
    green: Option<u32>,
    cyan: Option<u32>,
    blue: Option<u32>,
    purple: Option<u32>,
    magenta: Option<u32>,
}

// ── Code generation ────────────────────────────────────────────────

pub fn generate() -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("vim");
    let output_dir = project_root.join("src").join("vim");

    if !data_dir.exists() {
        anyhow::bail!("No data/vim/ directory found.\nRun `cargo xtask fetch vim` first.");
    }

    fs::create_dir_all(&output_dir).context("creating src/vim/")?;

    let mut generated_modules: Vec<(String, String)> = Vec::new();
    let mut emitted_names: HashSet<String> = HashSet::new();
    let mut skip_count = 0;

    let mut entries: Vec<_> = fs::read_dir(&data_dir)
        .context("reading data/vim/")?
        .filter_map(|e| e.ok())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("vim") {
            continue;
        }

        let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("  Skipping {}: {e}", path.display());
                skip_count += 1;
                continue;
            }
        };

        if should_skip(&content, file_size) {
            skip_count += 1;
            continue;
        }

        let file_stem = path.file_stem().unwrap().to_str().unwrap();

        let theme_data = match parse_vim_colorscheme(&content) {
            Some(t) => t,
            None => {
                skip_count += 1;
                continue;
            }
        };

        let theme_name = theme_data
            .name
            .clone()
            .unwrap_or_else(|| file_stem.to_string());

        // Skip duplicate names
        let name_key = theme_name.to_lowercase();
        if emitted_names.contains(&name_key) {
            skip_count += 1;
            continue;
        }
        emitted_names.insert(name_key);

        let module_name = sanitize_module_name(file_stem);
        let const_name = sanitize_const_name(&theme_name);

        let rust_code = generate_theme_file(&theme_data, &theme_name);
        let output_path = output_dir.join(format!("{module_name}.rs"));

        if write_if_changed(&output_path, &rust_code)? {
            println!("  Generated: src/vim/{module_name}.rs ({const_name})");
        }

        generated_modules.push((module_name, const_name));
    }

    generated_modules.sort_by(|a, b| a.0.cmp(&b.0));

    let mod_content = generate_mod_rs(&generated_modules);
    let mod_path = output_dir.join("mod.rs");
    write_if_changed(&mod_path, &mod_content)?;

    println!(
        "\nGenerated {} vim theme(s), skipped {skip_count}. Updated src/vim/mod.rs.",
        generated_modules.len()
    );
    Ok(())
}

fn hex_literal(hex: u32) -> String {
    format!("0x{:06x}", hex)
}

fn opt_color(val: Option<u32>) -> String {
    match val {
        Some(h) => format!("Some(Color::from_hex({}))", hex_literal(h)),
        None => "None".to_string(),
    }
}

fn generate_theme_file(theme: &VimThemeData, display_name: &str) -> String {
    // Use luminance-based detection rather than trusting the vim file's
    // `set background=` value, as some themes have incorrect metadata.
    let variant_enum = detect_variant(theme.bg);
    let contrast_enum = classify_contrast(theme.bg, theme.fg);
    let escaped_name = display_name.replace('"', "\\\"");

    format!(
        r#"//! {name} color theme.
//!
//! Auto-generated by `cargo xtask generate vim` — do not edit.

use crate::{{Color, Contrast, Theme, Variant}};
use alloc::borrow::Cow;

/// {name}
///
/// Variant: {variant_enum}
/// Contrast: {contrast_enum}
/// Source: vim (vim-colorschemes)
pub const THEME: Theme = Theme {{
    name: Cow::Borrowed("{escaped_name}"),
    author: Cow::Borrowed(""),
    variant: Variant::{variant_enum},
    contrast: Contrast::{contrast_enum},
    bg: Color::from_hex({bg}),
    fg: Color::from_hex({fg}),
    cursor: {cursor},
    selection: {selection},
    line_highlight: {line_highlight},
    gutter: {gutter},
    statusbar_bg: {statusbar_bg},
    statusbar_fg: {statusbar_fg},
    comment: {comment},
    keyword: {keyword},
    string: {string},
    function: {function},
    variable: {variable},
    r#type: {type_color},
    constant: {constant},
    operator: {operator},
    tag: {tag},
    error: {error},
    warning: {warning},
    info: None,
    success: None,
    red: {red},
    orange: {orange},
    yellow: {yellow},
    green: {green},
    cyan: {cyan},
    blue: {blue},
    purple: {purple},
    magenta: {magenta},
}};
"#,
        name = escaped_name,
        bg = hex_literal(theme.bg),
        fg = hex_literal(theme.fg),
        cursor = opt_color(theme.cursor),
        selection = opt_color(theme.selection),
        line_highlight = opt_color(theme.line_highlight),
        gutter = opt_color(theme.gutter),
        statusbar_bg = opt_color(theme.statusbar_bg),
        statusbar_fg = opt_color(theme.statusbar_fg),
        comment = opt_color(theme.comment),
        keyword = opt_color(theme.keyword),
        string = opt_color(theme.string),
        function = opt_color(theme.function),
        variable = opt_color(theme.variable),
        type_color = opt_color(theme.r#type),
        constant = opt_color(theme.constant),
        operator = opt_color(theme.operator),
        tag = opt_color(theme.tag),
        error = opt_color(theme.error),
        warning = opt_color(theme.warning),
        red = opt_color(theme.red),
        orange = opt_color(theme.orange),
        yellow = opt_color(theme.yellow),
        green = opt_color(theme.green),
        cyan = opt_color(theme.cyan),
        blue = opt_color(theme.blue),
        purple = opt_color(theme.purple),
        magenta = opt_color(theme.magenta),
    )
}

fn generate_mod_rs(modules: &[(String, String)]) -> String {
    let mut output = String::from(
        "//! Vim color themes.\n\
         //!\n\
         //! Themes parsed from [vim-colorschemes](https://github.com/flazz/vim-colorschemes)\n\
         //! and [vim/colorschemes](https://github.com/vim/colorschemes).\n\
         //! Colors extracted from `hi` (highlight) commands in `.vim` files.\n\
         //!\n\
         //! Enable with the `vim` feature flag.\n\
         //!\n\
         //! Auto-generated by `cargo xtask generate vim` — do not edit.\n\n\
         use crate::Theme;\n\n",
    );

    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {module_name};\n"));
    }

    output.push_str("\n/// All themes in the `vim` collection.\n");
    output.push_str("pub static THEMES: &[&Theme] = &[\n");
    for (module_name, _) in modules {
        output.push_str(&format!("    &{module_name}::THEME,\n"));
    }
    output.push_str("];\n");

    output
}
