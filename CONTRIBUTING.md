# Contributing to Chromata

Thank you for your interest in contributing to Chromata. This document covers
the essentials for reporting bugs, submitting changes, and meeting code quality
requirements.

## Reporting Bugs

Open an issue at <https://github.com/resonant-jovian/chromata/issues> with:

- A clear description of the problem
- Steps to reproduce
- Expected vs. actual behavior
- Rust version (`rustc --version`) and OS

## Pull Request Process

1. Fork the repository and create a feature branch from `main`.
2. Make your changes (see requirements below).
3. Open a pull request against `main` with a concise description of the change.
4. Address any review feedback.

## Development Setup

```bash
git clone https://github.com/resonant-jovian/chromata.git
cd chromata
cargo xtask fetch all      # Download upstream theme data
cargo xtask generate all   # Generate Rust source from data/
cargo test --all-features -- --test-threads=1
```

MSRV is **1.85**. Ensure your toolchain meets this minimum.

## Code Quality Requirements

Before submitting a PR, verify all of the following pass:

```bash
cargo fmt --all                                            # Format
cargo clippy --all-targets --all-features -- -D warnings   # Lint (library)
cargo clippy -p xtask -- -D warnings                       # Lint (xtask)
cargo test --all-features -- --test-threads=1              # Tests
cargo check --no-default-features                          # no_std compat
```

Or use the development script:

```bash
./dev.sh ci    # Runs all of the above plus more
```

The crate enforces:

- `#![no_std]` -- all library code must work without the standard library.
- `unsafe_code = "forbid"` -- no `unsafe` blocks, ever.
- `clippy::unwrap_used = "deny"` -- use `expect()` or proper error handling in library code.

## Generated Files

Files in `src/base16/`, `src/base24/`, `src/vim/`, and `src/emacs/` are
generated from upstream data. **Do not edit these files by hand.** Instead,
modify the code generation pipeline in the `xtask/` crate and re-run:

```bash
cargo xtask fetch all && cargo xtask generate all
```

Hand-curated themes in `src/popular/` are manually maintained and can be
edited directly.

## License

By contributing, you agree that your contributions will be licensed under the
GPL-3.0-only license, the same license that covers the project.
