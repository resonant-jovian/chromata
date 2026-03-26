use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

use crate::codegen::project_root;

pub fn fetch(force: bool) -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("emacs");
    fs::create_dir_all(&data_dir).context("creating data/emacs/")?;

    // Check if we already have data and skip unless --force
    let existing_count = fs::read_dir(&data_dir)
        .map(|rd| rd.filter(|e| e.as_ref().is_ok_and(|e| {
            e.path().extension().and_then(|x| x.to_str()) == Some("el")
        })).count())
        .unwrap_or(0);

    if existing_count > 0 && !force {
        println!("data/emacs/ already contains {existing_count} files (use --force to re-fetch)");
        return Ok(());
    }

    let tmp_dir = project_root.join(".xtask-tmp-emacs");
    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).context("cleaning up old emacs temp dir")?;
    }

    println!("Cloning emacs-themes-site...");
    let urls = [
        "https://github.com/srdja/emacs-themes-site.git",
        "https://github.com/emacs-themes/emacs-themes-site.git",
    ];
    let mut cloned = false;
    for url in &urls {
        let status = Command::new("git")
            .args(["clone", "--depth", "1", url])
            .arg(&tmp_dir)
            .status()
            .with_context(|| format!("running git clone for {url}"))?;
        if status.success() {
            cloned = true;
            break;
        }
        // Clean up failed clone attempt
        if tmp_dir.exists() {
            let _ = fs::remove_dir_all(&tmp_dir);
        }
    }
    if !cloned {
        anyhow::bail!("git clone failed for emacs-themes-site (tried all known URLs)");
    }

    let src_dir = tmp_dir.join("root").join("assets").join("local-src");
    if !src_dir.exists() {
        // Try alternate structure
        let alt = tmp_dir.join("assets").join("local-src");
        if !alt.exists() {
            anyhow::bail!(
                "Could not find theme source files in cloned repo (tried root/assets/local-src/ and assets/local-src/)"
            );
        }
        return copy_el_files(&alt, &data_dir, &tmp_dir);
    }

    copy_el_files(&src_dir, &data_dir, &tmp_dir)
}

fn copy_el_files(
    src_dir: &std::path::Path,
    data_dir: &std::path::Path,
    tmp_dir: &std::path::Path,
) -> Result<()> {
    let mut count = 0;
    for entry in fs::read_dir(src_dir).with_context(|| format!("reading {}", src_dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("el") {
            continue;
        }

        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        // Strip "-theme-source-code" suffix if present
        let clean_name = file_name
            .strip_suffix("-theme-source-code")
            .unwrap_or(file_name);

        let dest = data_dir.join(format!("{clean_name}.el"));
        fs::copy(&path, &dest)
            .with_context(|| format!("copying {}", path.display()))?;
        count += 1;
    }

    // Cleanup
    if tmp_dir.exists() {
        let _ = fs::remove_dir_all(tmp_dir);
    }

    println!("Fetched {count} emacs theme files to data/emacs/");
    Ok(())
}
