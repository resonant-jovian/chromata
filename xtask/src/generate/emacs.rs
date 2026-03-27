use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;

use crate::codegen::{
    classify_contrast, detect_variant, project_root, sanitize_const_name, sanitize_module_name,
    write_if_changed,
};

// ── S-expression tokenizer ──────────────────────────────────────────

#[derive(Debug, Clone)]
enum Token {
    Open,
    Close,
    Quote,
    String(String),
    Symbol(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        match chars[i] {
            // Skip whitespace
            c if c.is_whitespace() => i += 1,
            // Skip comments
            ';' => {
                while i < len && chars[i] != '\n' {
                    i += 1;
                }
            }
            '(' => {
                tokens.push(Token::Open);
                i += 1;
            }
            ')' => {
                tokens.push(Token::Close);
                i += 1;
            }
            '\'' => {
                tokens.push(Token::Quote);
                i += 1;
            }
            '`' => {
                // Backquote — treat like quote for our purposes
                tokens.push(Token::Quote);
                i += 1;
            }
            ',' => {
                // Unquote in backquoted forms — skip
                i += 1;
                if i < len && chars[i] == '@' {
                    i += 1;
                }
            }
            '"' => {
                // String literal
                i += 1;
                let mut s = String::new();
                while i < len && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < len {
                        i += 1;
                        match chars[i] {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            other => {
                                s.push('\\');
                                s.push(other);
                            }
                        }
                    } else {
                        s.push(chars[i]);
                    }
                    i += 1;
                }
                if i < len {
                    i += 1; // skip closing "
                }
                tokens.push(Token::String(s));
            }
            _ => {
                // Symbol or number
                let start = i;
                while i < len
                    && !chars[i].is_whitespace()
                    && chars[i] != '('
                    && chars[i] != ')'
                    && chars[i] != '"'
                    && chars[i] != ';'
                {
                    i += 1;
                }
                let sym: String = chars[start..i].iter().collect();
                if !sym.is_empty() {
                    tokens.push(Token::Symbol(sym));
                }
            }
        }
    }

    tokens
}

// ── Face extraction ────────────────────────────────────────────────

struct FaceDef {
    name: String,
    foreground: Option<String>,
    background: Option<String>,
}

fn extract_faces(tokens: &[Token]) -> Vec<FaceDef> {
    let mut faces = Vec::new();

    // Find custom-theme-set-faces or custom-set-faces
    let mut i = 0;
    while i < tokens.len() {
        if let Token::Symbol(ref s) = tokens[i]
            && (s == "custom-theme-set-faces" || s == "custom-set-faces")
        {
            i += 1;
            // Skip the theme name symbol that follows
            while i < tokens.len() {
                match &tokens[i] {
                    Token::Quote | Token::Symbol(_) => i += 1,
                    _ => break,
                }
            }
            // Now extract face definitions
            extract_face_definitions(tokens, &mut i, &mut faces);
            break;
        }
        i += 1;
    }

    faces
}

fn extract_face_definitions(tokens: &[Token], i: &mut usize, faces: &mut Vec<FaceDef>) {
    while *i < tokens.len() {
        // Skip Quote tokens
        while *i < tokens.len() && matches!(tokens[*i], Token::Quote) {
            *i += 1;
        }

        if *i >= tokens.len() {
            break;
        }

        match &tokens[*i] {
            Token::Open => {
                // Start of a face definition: (face-name ((display-spec (:prop val ...))))
                *i += 1;
                if let Some(face) = parse_one_face(tokens, i) {
                    faces.push(face);
                } else {
                    // Skip to matching close
                    skip_to_close(tokens, i);
                }
            }
            Token::Close => {
                // End of the set-faces form
                *i += 1;
                break;
            }
            _ => *i += 1,
        }
    }
}

fn parse_one_face(tokens: &[Token], i: &mut usize) -> Option<FaceDef> {
    // We're just past the opening paren
    // Expect: face-name then property specs, then close

    let face_name = match tokens.get(*i)? {
        Token::Symbol(s) => s.clone(),
        _ => return None,
    };
    *i += 1;

    let mut fg = None;
    let mut bg = None;

    // Scan through tokens until we hit the matching close paren
    let mut depth = 0;
    while *i < tokens.len() {
        match &tokens[*i] {
            Token::Open => {
                depth += 1;
                *i += 1;
            }
            Token::Close => {
                if depth == 0 {
                    *i += 1;
                    break;
                }
                depth -= 1;
                *i += 1;
            }
            Token::Symbol(s) if s == ":foreground" => {
                *i += 1;
                if let Some(Token::String(val)) = tokens.get(*i) {
                    fg = Some(val.clone());
                    *i += 1;
                }
            }
            Token::Symbol(s) if s == ":background" => {
                *i += 1;
                if let Some(Token::String(val)) = tokens.get(*i) {
                    bg = Some(val.clone());
                    *i += 1;
                }
            }
            _ => *i += 1,
        }
    }

    Some(FaceDef {
        name: face_name,
        foreground: fg,
        background: bg,
    })
}

fn skip_to_close(tokens: &[Token], i: &mut usize) {
    let mut depth = 0;
    while *i < tokens.len() {
        match tokens[*i] {
            Token::Open => depth += 1,
            Token::Close => {
                if depth == 0 {
                    *i += 1;
                    return;
                }
                depth -= 1;
            }
            _ => {}
        }
        *i += 1;
    }
}

// ── Color resolution ───────────────────────────────────────────────

fn resolve_color(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix('#') {
        if hex.len() == 6 {
            return u32::from_str_radix(hex, 16).ok();
        }
        if hex.len() == 3 {
            // Expand #RGB to #RRGGBB
            let chars: Vec<char> = hex.chars().collect();
            let expanded = format!("{0}{0}{1}{1}{2}{2}", chars[0], chars[1], chars[2]);
            return u32::from_str_radix(&expanded, 16).ok();
        }
        return None;
    }
    // X11 named color lookup (case-insensitive)
    x11_color(s)
}

fn x11_color(name: &str) -> Option<u32> {
    let lower = name.to_lowercase().replace(' ', "");
    X11_COLORS
        .iter()
        .find(|(n, _)| *n == lower)
        .map(|(_, c)| *c)
}

/// Common X11 named colors used in emacs themes.
static X11_COLORS: &[(&str, u32)] = &[
    ("aliceblue", 0xf0f8ff),
    ("antiquewhite", 0xfaebd7),
    ("aqua", 0x00ffff),
    ("aquamarine", 0x7fffd4),
    ("azure", 0xf0ffff),
    ("beige", 0xf5f5dc),
    ("bisque", 0xffe4c4),
    ("black", 0x000000),
    ("blanchedalmond", 0xffebcd),
    ("blue", 0x0000ff),
    ("blueviolet", 0x8a2be2),
    ("brown", 0xa52a2a),
    ("burlywood", 0xdeb887),
    ("cadetblue", 0x5f9ea0),
    ("chartreuse", 0x7fff00),
    ("chocolate", 0xd2691e),
    ("coral", 0xff7f50),
    ("cornflowerblue", 0x6495ed),
    ("cornsilk", 0xfff8dc),
    ("crimson", 0xdc143c),
    ("cyan", 0x00ffff),
    ("darkblue", 0x00008b),
    ("darkcyan", 0x008b8b),
    ("darkgoldenrod", 0xb8860b),
    ("darkgray", 0xa9a9a9),
    ("darkgreen", 0x006400),
    ("darkgrey", 0xa9a9a9),
    ("darkkhaki", 0xbdb76b),
    ("darkmagenta", 0x8b008b),
    ("darkolivegreen", 0x556b2f),
    ("darkorange", 0xff8c00),
    ("darkorchid", 0x9932cc),
    ("darkred", 0x8b0000),
    ("darksalmon", 0xe9967a),
    ("darkseagreen", 0x8fbc8f),
    ("darkslateblue", 0x483d8b),
    ("darkslategray", 0x2f4f4f),
    ("darkslategrey", 0x2f4f4f),
    ("darkturquoise", 0x00ced1),
    ("darkviolet", 0x9400d3),
    ("deeppink", 0xff1493),
    ("deepskyblue", 0x00bfff),
    ("dimgray", 0x696969),
    ("dimgrey", 0x696969),
    ("dodgerblue", 0x1e90ff),
    ("firebrick", 0xb22222),
    ("floralwhite", 0xfffaf0),
    ("forestgreen", 0x228b22),
    ("fuchsia", 0xff00ff),
    ("gainsboro", 0xdcdcdc),
    ("ghostwhite", 0xf8f8ff),
    ("gold", 0xffd700),
    ("goldenrod", 0xdaa520),
    ("gray", 0x808080),
    ("green", 0x008000),
    ("greenyellow", 0xadff2f),
    ("grey", 0x808080),
    ("honeydew", 0xf0fff0),
    ("hotpink", 0xff69b4),
    ("indianred", 0xcd5c5c),
    ("indigo", 0x4b0082),
    ("ivory", 0xfffff0),
    ("khaki", 0xf0e68c),
    ("lavender", 0xe6e6fa),
    ("lavenderblush", 0xfff0f5),
    ("lawngreen", 0x7cfc00),
    ("lemonchiffon", 0xfffacd),
    ("lightblue", 0xadd8e6),
    ("lightcoral", 0xf08080),
    ("lightcyan", 0xe0ffff),
    ("lightgoldenrodyellow", 0xfafad2),
    ("lightgray", 0xd3d3d3),
    ("lightgreen", 0x90ee90),
    ("lightgrey", 0xd3d3d3),
    ("lightpink", 0xffb6c1),
    ("lightsalmon", 0xffa07a),
    ("lightseagreen", 0x20b2aa),
    ("lightskyblue", 0x87cefa),
    ("lightslategray", 0x778899),
    ("lightslategrey", 0x778899),
    ("lightsteelblue", 0xb0c4de),
    ("lightyellow", 0xffffe0),
    ("lime", 0x00ff00),
    ("limegreen", 0x32cd32),
    ("linen", 0xfaf0e6),
    ("magenta", 0xff00ff),
    ("maroon", 0x800000),
    ("mediumaquamarine", 0x66cdaa),
    ("mediumblue", 0x0000cd),
    ("mediumorchid", 0xba55d3),
    ("mediumpurple", 0x9370db),
    ("mediumseagreen", 0x3cb371),
    ("mediumslateblue", 0x7b68ee),
    ("mediumspringgreen", 0x00fa9a),
    ("mediumturquoise", 0x48d1cc),
    ("mediumvioletred", 0xc71585),
    ("midnightblue", 0x191970),
    ("mintcream", 0xf5fffa),
    ("mistyrose", 0xffe4e1),
    ("moccasin", 0xffe4b5),
    ("navajowhite", 0xffdead),
    ("navy", 0x000080),
    ("oldlace", 0xfdf5e6),
    ("olive", 0x808000),
    ("olivedrab", 0x6b8e23),
    ("orange", 0xffa500),
    ("orangered", 0xff4500),
    ("orchid", 0xda70d6),
    ("palegoldenrod", 0xeee8aa),
    ("palegreen", 0x98fb98),
    ("paleturquoise", 0xafeeee),
    ("palevioletred", 0xdb7093),
    ("papayawhip", 0xffefd5),
    ("peachpuff", 0xffdab9),
    ("peru", 0xcd853f),
    ("pink", 0xffc0cb),
    ("plum", 0xdda0dd),
    ("powderblue", 0xb0e0e6),
    ("purple", 0x800080),
    ("red", 0xff0000),
    ("rosybrown", 0xbc8f8f),
    ("royalblue", 0x4169e1),
    ("saddlebrown", 0x8b4513),
    ("salmon", 0xfa8072),
    ("sandybrown", 0xf4a460),
    ("seagreen", 0x2e8b57),
    ("seashell", 0xfff5ee),
    ("sienna", 0xa0522d),
    ("silver", 0xc0c0c0),
    ("skyblue", 0x87ceeb),
    ("slateblue", 0x6a5acd),
    ("slategray", 0x708090),
    ("slategrey", 0x708090),
    ("snow", 0xfffafa),
    ("springgreen", 0x00ff7f),
    ("steelblue", 0x4682b4),
    ("tan", 0xd2b48c),
    ("teal", 0x008080),
    ("thistle", 0xd8bfd8),
    ("tomato", 0xff6347),
    ("turquoise", 0x40e0d0),
    ("violet", 0xee82ee),
    ("wheat", 0xf5deb3),
    ("white", 0xffffff),
    ("whitesmoke", 0xf5f5f5),
    ("yellow", 0xffff00),
    ("yellowgreen", 0x9acd32),
    // Numbered variants commonly used in emacs themes
    ("gray10", 0x1a1a1a),
    ("gray15", 0x262626),
    ("gray20", 0x333333),
    ("gray25", 0x404040),
    ("gray30", 0x4d4d4d),
    ("gray35", 0x595959),
    ("gray40", 0x666666),
    ("gray50", 0x808080),
    ("gray60", 0x999999),
    ("gray70", 0xb3b3b3),
    ("gray75", 0xbfbfbf),
    ("gray80", 0xcccccc),
    ("gray85", 0xd9d9d9),
    ("gray90", 0xe5e5e5),
    ("gray95", 0xf2f2f2),
    ("grey10", 0x1a1a1a),
    ("grey15", 0x262626),
    ("grey20", 0x333333),
    ("grey25", 0x404040),
    ("grey30", 0x4d4d4d),
    ("grey35", 0x595959),
    ("grey40", 0x666666),
    ("grey50", 0x808080),
    ("grey60", 0x999999),
    ("grey70", 0xb3b3b3),
    ("grey75", 0xbfbfbf),
    ("grey80", 0xcccccc),
    ("grey85", 0xd9d9d9),
    ("grey90", 0xe5e5e5),
    ("grey95", 0xf2f2f2),
];

// ── Theme mapping ──────────────────────────────────────────────────

struct ThemeData {
    name: String,
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
    error: Option<u32>,
    warning: Option<u32>,
    success: Option<u32>,
}

fn map_faces_to_theme(faces: &[FaceDef], name: &str) -> Option<ThemeData> {
    let face_map: HashMap<&str, &FaceDef> = faces.iter().map(|f| (f.name.as_str(), f)).collect();

    let bg = face_map
        .get("default")
        .and_then(|f| f.background.as_ref())
        .and_then(|s| resolve_color(s))?;
    let fg = face_map
        .get("default")
        .and_then(|f| f.foreground.as_ref())
        .and_then(|s| resolve_color(s))?;

    if bg == fg {
        return None;
    }

    let get_fg = |face_name: &str| -> Option<u32> {
        face_map
            .get(face_name)
            .and_then(|f| f.foreground.as_ref())
            .and_then(|s| resolve_color(s))
    };
    let get_bg = |face_name: &str| -> Option<u32> {
        face_map
            .get(face_name)
            .and_then(|f| f.background.as_ref())
            .and_then(|s| resolve_color(s))
    };

    let comment =
        get_fg("font-lock-comment-face").or_else(|| get_fg("font-lock-comment-delimiter-face"));
    let keyword = get_fg("font-lock-keyword-face");
    let string = get_fg("font-lock-string-face");
    let function = get_fg("font-lock-function-name-face");
    let variable = get_fg("font-lock-variable-name-face");
    let type_color = get_fg("font-lock-type-face");
    let constant = get_fg("font-lock-constant-face");

    // Quality gate: bg + fg + at least 4 syntax colors
    let syntax_count = [
        comment, keyword, string, function, variable, type_color, constant,
    ]
    .iter()
    .filter(|c| c.is_some())
    .count();

    if syntax_count < 4 {
        return None;
    }

    Some(ThemeData {
        name: name.to_string(),
        bg,
        fg,
        cursor: get_bg("cursor").or_else(|| get_fg("cursor")),
        selection: get_bg("region"),
        line_highlight: get_bg("hl-line"),
        gutter: get_fg("line-number").or_else(|| get_fg("linum")),
        statusbar_bg: get_bg("mode-line"),
        statusbar_fg: get_fg("mode-line"),
        comment,
        keyword,
        string,
        function,
        variable,
        r#type: type_color,
        constant,
        error: get_fg("error"),
        warning: get_fg("warning"),
        success: get_fg("success"),
    })
}

// ── Code generation ────────────────────────────────────────────────

pub fn generate() -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("emacs");
    let output_dir = project_root.join("src").join("emacs");

    if !data_dir.exists() {
        anyhow::bail!("No data/emacs/ directory found.\nRun `cargo xtask fetch emacs` first.");
    }

    fs::create_dir_all(&output_dir).context("creating src/emacs/")?;

    let mut generated_modules: Vec<(String, String)> = Vec::new();
    let mut skip_count = 0;

    for entry in fs::read_dir(&data_dir).context("reading data/emacs/")? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("el") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("  Skipping {}: {e}", path.display());
                skip_count += 1;
                continue;
            }
        };

        let file_stem = path.file_stem().unwrap().to_str().unwrap();

        let tokens = tokenize(&content);
        let faces = extract_faces(&tokens);

        if faces.is_empty() {
            skip_count += 1;
            continue;
        }

        let theme_name = file_stem
            .replace(['-', '_'], " ")
            .split_whitespace()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(first) => {
                        let upper: String = first.to_uppercase().collect();
                        upper + c.as_str()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let theme_data = match map_faces_to_theme(&faces, &theme_name) {
            Some(t) => t,
            None => {
                skip_count += 1;
                continue;
            }
        };

        let module_name = sanitize_module_name(file_stem);
        let const_name = sanitize_const_name(&theme_name);

        let rust_code = generate_theme_file(&theme_data);
        let output_path = output_dir.join(format!("{module_name}.rs"));

        if write_if_changed(&output_path, &rust_code)? {
            println!("  Generated: src/emacs/{module_name}.rs ({const_name})");
        }

        generated_modules.push((module_name, const_name));
    }

    generated_modules.sort_by(|a, b| a.0.cmp(&b.0));

    let mod_content = generate_mod_rs(&generated_modules);
    let mod_path = output_dir.join("mod.rs");
    write_if_changed(&mod_path, &mod_content)?;

    println!(
        "\nGenerated {} emacs theme(s), skipped {skip_count}. Updated src/emacs/mod.rs.",
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

fn generate_theme_file(theme: &ThemeData) -> String {
    let variant_enum = detect_variant(theme.bg);
    let contrast_enum = classify_contrast(theme.bg, theme.fg);
    let escaped_name = theme.name.replace('"', "\\\"");

    // Derive accent colors from syntax colors
    let red = theme.error;
    let orange = None; // No obvious mapping from emacs faces
    let yellow = theme.warning;
    let green = theme.string.or(theme.success);
    let cyan = theme.constant.or(theme.r#type);
    let blue = theme.function;
    let purple = theme.keyword;
    let magenta = theme.variable;

    format!(
        r#"//! {name} color theme.
//!
//! Auto-generated by `cargo xtask generate emacs` — do not edit.

use crate::{{Color, Contrast, Theme, Variant}};

/// {name}
///
/// Variant: {variant_enum}
/// Contrast: {contrast_enum}
/// Source: emacs (emacs-themes-site)
pub const THEME: Theme = Theme {{
    name: "{escaped_name}",
    author: "",
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
    operator: None,
    tag: None,
    error: {error},
    warning: {warning},
    info: None,
    success: {success},
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
        error = opt_color(theme.error),
        warning = opt_color(theme.warning),
        success = opt_color(theme.success),
        red = opt_color(red),
        orange = opt_color(orange),
        yellow = opt_color(yellow),
        green = opt_color(green),
        cyan = opt_color(cyan),
        blue = opt_color(blue),
        purple = opt_color(purple),
        magenta = opt_color(magenta),
    )
}

fn generate_mod_rs(modules: &[(String, String)]) -> String {
    let mut output = String::from(
        "//! Emacs color themes.\n\
         //!\n\
         //! Themes parsed from [emacs-themes-site](https://github.com/emacs-themes/emacs-themes-site).\n\
         //! Colors extracted from `custom-theme-set-faces` definitions in `.el` files.\n\
         //!\n\
         //! Enable with the `emacs` feature flag.\n\
         //!\n\
         //! Auto-generated by `cargo xtask generate emacs` — do not edit.\n\n\
         use crate::Theme;\n\n",
    );

    for (module_name, _) in modules {
        output.push_str(&format!("pub mod {module_name};\n"));
    }

    output.push_str("\n/// All themes in the `emacs` collection.\n");
    output.push_str("pub static THEMES: &[&Theme] = &[\n");
    for (module_name, _) in modules {
        output.push_str(&format!("    &{module_name}::THEME,\n"));
    }
    output.push_str("];\n");

    output
}
