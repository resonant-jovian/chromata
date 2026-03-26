# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-03-26

### Added

- **base24 collection**: 184 themes from tinted-theming/schemes (feature: `base24`)
- **emacs collection**: 102 themes parsed from emacs-themes-site `.el` files (feature: `emacs`)
- **vim collection**: 464 themes parsed from flazz/vim-colorschemes and vim/colorschemes (feature: `vim`)
- `find_by_name()` — look up a theme by exact name across all enabled collections
- `filter_by_variant()` — filter themes by Dark/Light variant
- `filter_by_contrast()` — filter themes by High/Normal/Low contrast level
- `prelude` module — convenience re-exports of common types
- `Display` impl for `Color` (formats as `#rrggbb`), `Variant`, and `Contrast`
- Property tests with `proptest` for Color roundtrips, contrast symmetry, luminance bounds
- Per-collection test suites (count thresholds, no duplicates, bg/fg distinct)
- `cargo xtask check` — verify generated files match committed code
- `cargo xtask ci` — full pipeline: fetch, generate, clippy, test
- `cargo xtask clean` — remove cached data and/or generated files

### Changed

- Xtask restructured from monolithic `main.rs` into per-collection modules with `clap` CLI
- Xtask now supports `fetch`/`generate` subcommands for individual collections or `all`
- Variant detection for generated themes uses luminance-based calculation instead of trusting upstream metadata

## [0.1.0] - 2026-03-20

### Added

- Initial release
- **popular collection**: 48 hand-crafted themes from 24 families (feature: `popular`, default)
- **base16 collection**: 305 auto-generated themes from tinted-theming/schemes (feature: `base16`)
- Core types: `Color`, `Theme`, `Variant`, `Contrast`, `Base16Palette`, `Base24Palette`
- Framework integrations: ratatui, egui, crossterm, iced
- WCAG contrast ratio utilities (`luminance()`, `contrast_ratio()`)
- `no_std` compatible with `alloc` for Vec/String operations
- `collect_all_themes()` aggregation function
- `cargo xtask fetch` / `cargo xtask generate` for base16 code generation
