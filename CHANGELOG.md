# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org/).

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

[0.3.0]: https://github.com/resonant-jovian/chromata/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/resonant-jovian/chromata/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/resonant-jovian/chromata/releases/tag/0.1.0
