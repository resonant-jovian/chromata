pub mod base16;
pub mod base24;
pub mod emacs;
pub mod vim;

use anyhow::{Result, bail};

/// Generate Rust source files from fetched data for a given collection.
pub fn generate(collection: &str) -> Result<()> {
    match collection {
        "base16" => base16::generate(),
        "base24" => base24::generate(),
        "vim" => vim::generate(),
        "emacs" => emacs::generate(),
        "all" => {
            base16::generate()?;
            base24::generate()?;
            vim::generate()?;
            emacs::generate()?;
            Ok(())
        }
        other => bail!("Unknown collection: {other}"),
    }
}
