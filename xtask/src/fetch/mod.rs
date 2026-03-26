pub mod base16;
pub mod base24;
pub mod emacs;
pub mod vim;

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::codegen::project_root;

/// Fetch upstream theme data for a given collection.
pub fn fetch(collection: &str, force: bool) -> Result<()> {
    match collection {
        "base16" => base16::fetch(force),
        "base24" => base24::fetch(force),
        "vim" => vim::fetch(force),
        "emacs" => emacs::fetch(force),
        "all" => {
            base16::fetch(force)?;
            base24::fetch(force)?;
            vim::fetch(force)?;
            emacs::fetch(force)?;
            Ok(())
        }
        other => bail!("Unknown collection: {other}"),
    }
}

/// Clone (or reuse) the tinted-theming/schemes repository.
///
/// Returns the path to the cloned directory. Both base16 and base24
/// share this repo, so we only clone once per `fetch all` run.
pub fn clone_tinted_theming(force: bool) -> Result<PathBuf> {
    let project_root = project_root();
    let tmp_dir = project_root.join(".xtask-tmp-schemes");

    if tmp_dir.exists() && !force {
        return Ok(tmp_dir);
    }

    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).context("cleaning up old temp dir")?;
    }

    println!("Cloning tinted-theming/schemes...");
    let status = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "https://github.com/tinted-theming/schemes.git",
        ])
        .arg(&tmp_dir)
        .status()
        .context("running git clone — is git installed?")?;

    if !status.success() {
        bail!("git clone failed with exit code: {status}");
    }

    Ok(tmp_dir)
}

/// Clean up the tinted-theming temp directory if it exists.
pub fn cleanup_tinted_theming() {
    let tmp_dir = project_root().join(".xtask-tmp-schemes");
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(&tmp_dir);
    }
}
