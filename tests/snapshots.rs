#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::Path;

fn read_first_n_lines(path: &Path, n: usize) -> String {
    let content = fs::read_to_string(path).unwrap();
    content.lines().take(n).collect::<Vec<_>>().join("\n")
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

// --- Mod headers (first 20 lines) ---

#[test]
fn snapshot_base16_mod_header() {
    let content = read_first_n_lines(&root().join("src/base16/mod.rs"), 20);
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_base24_mod_header() {
    let content = read_first_n_lines(&root().join("src/base24/mod.rs"), 20);
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_vim_mod_header() {
    let content = read_first_n_lines(&root().join("src/vim/mod.rs"), 20);
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_emacs_mod_header() {
    let content = read_first_n_lines(&root().join("src/emacs/mod.rs"), 20);
    insta::assert_snapshot!(content);
}

// --- Representative theme files (full content) ---

#[test]
fn snapshot_base16_gruvbox_dark_hard() {
    let content = read_file(&root().join("src/base16/gruvbox_dark_hard.rs"));
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_base24_adventure_time() {
    let content = read_file(&root().join("src/base24/adventure_time.rs"));
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_vim_monokai() {
    let content = read_file(&root().join("src/vim/monokai.rs"));
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_emacs_dracula() {
    let content = read_file(&root().join("src/emacs/dracula.rs"));
    insta::assert_snapshot!(content);
}

#[test]
fn snapshot_popular_gruvbox() {
    let content = read_file(&root().join("src/popular/gruvbox.rs"));
    insta::assert_snapshot!(content);
}
