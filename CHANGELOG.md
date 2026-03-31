# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.0.0] - 2026-03-31

### Breaking Changes

- **Theme `name`/`author` fields changed from `&'static str` to `Cow<'static, str>`** —
  enables serde deserialization. Compile-time themes use `Cow::Borrowed` (zero-cost).
- **Theme no longer implements `Copy`** — use `Clone` or references instead.
  Theme is a large struct; implicit copies were arguably misleading.
- **Theme is now `#[non_exhaustive]`** — external crates must use `Theme::builder()`
  to construct themes. This allows adding new fields in future minor versions.
- **serde-support now derives both `Serialize` and `Deserialize` on `Theme`** —
  previously serialize-only.
- **Integration convenience methods renamed** for consistent naming convention
  (`to_*` for conversions, `apply_to_*` for mutations):
  - `colorize()` → `to_colored_string()`
  - `style_comfy_cell()` → `to_comfy_table_cell()`
  - `plotters_series_colors()` → `to_plotters_series_colors()`
  - `to_syntect_theme_settings()` → `to_syntect_settings()`
  - `apply_to_visuals()` → `apply_to_egui_visuals()`

### Added

- `Theme::builder()` / `ThemeBuilder` for constructing themes at runtime with
  auto-detected variant (from bg luminance) and contrast (from bg/fg WCAG ratio)
- `Contrast::from_ratio(f64)` to classify WCAG contrast ratios
- `FromStr` for `Variant` ("Dark"/"Light") and `Contrast` ("High"/"Normal"/"Low"),
  case-insensitive
- `Default` for `Variant` (Dark) and `Contrast` (Normal)
- `From<u32>` and `From<(u8, u8, u8)>` for `Color`
- `FromStr` for `Color` (parses CSS hex strings)
- 3-digit CSS hex support in `Color::from_css_hex()` (e.g., `#FFF` expands to `#FFFFFF`)
- `PartialOrd` and `Ord` derives on `Color` (lexicographic by r, g, b)
- `Default` for `Color` (black)
- `Color::from_f32(r, g, b)` to construct from normalized `[0.0, 1.0]` components
- `From<[u8; 3]>` and `From<Color> for [u8; 3]` array conversions
- `#[must_use]` annotations on all value-returning methods and builder types
- Criterion benchmarks for color conversions, WCAG calculations, and theme operations
- 12 new integration examples (one per framework)
- WASM compatibility check in CI
- `cargo-semver-checks` CI job for semver compliance
- `SECURITY.md`, `CODE_OF_CONDUCT.md`, issue/PR templates
- `cargo-deny` for dependency auditing (licenses, advisories, sources)

## [0.3.1] - 2026-03-28

### Fixed

- Preamble check logic for generated files
- Test and code quality improvements

## [0.3.0] - 2026-03-28

### Added

- 14 new framework integrations: bevy_color, colored, comfy-table, cursive, image,
  macroquad, owo-colors, palette, plotters, slint, syntect, termion, tiny-skia, wgpu
- Convenience methods on `Theme`: `plotters_series_colors()`, `colorize()`,
  `style_comfy_cell()`, `apply_to_cursive_palette()`, `to_syntect_theme_settings()`
- 4 new examples: `plotters_chart`, `image_gradient`, `colored_terminal`, `comfy_table_demo`
- Framework integration test suite (33 tests in `tests/framework_integrations.rs`)
- Enriched inline docs with Enable/Example/Convenience sections for all 18 integrations

### Removed

- `IntoFrameworkColor` and `IntoFrameworkTheme` traits — replaced by direct `From<Color>` implementations

## [0.2.0] - 2025-05-01

### Added

- 4 framework integrations: ratatui, egui, crossterm, iced
- `serde` support via `serde-support` feature

## [0.1.0] - 2025-05-01

### Added

- 1104 color themes across 5 families: popular (49), base16 (305), base24 (184),
  vim (464), emacs (102)
- Core types: `Color`, `Theme`, `Variant`, `Contrast`, `Base16Palette`, `Base24Palette`
- WCAG contrast utilities: `luminance()`, `contrast_ratio()`, `Contrast` enum
- `no_std` compatibility with `extern crate alloc`
- Query APIs: `collect_all_themes()`, `find_by_name()`, `filter_by_variant()`,
  `filter_by_contrast()`

[1.0.0]: https://github.com/resonant-jovian/chromata/compare/0.3.1...1.0.0
[0.3.1]: https://github.com/resonant-jovian/chromata/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/resonant-jovian/chromata/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/resonant-jovian/chromata/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/resonant-jovian/chromata/releases/tag/0.1.0
