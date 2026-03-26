use anyhow::{Context, Result};
use std::fs;
use std::process::Command;

use crate::codegen::project_root;

pub fn fetch(force: bool) -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("vim");
    fs::create_dir_all(&data_dir).context("creating data/vim/")?;

    // Check if we already have data and skip unless --force
    let existing_count = fs::read_dir(&data_dir)
        .map(|rd| {
            rd.filter(|e| {
                e.as_ref()
                    .is_ok_and(|e| e.path().extension().and_then(|x| x.to_str()) == Some("vim"))
            })
            .count()
        })
        .unwrap_or(0);

    if existing_count > 0 && !force {
        println!("data/vim/ already contains {existing_count} files (use --force to re-fetch)");
        return Ok(());
    }

    let tmp_base = project_root.join(".xtask-tmp-vim");

    // Source 1: flazz/vim-colorschemes (large collection)
    let flazz_dir = tmp_base.join("flazz");
    clone_repo(
        "https://github.com/flazz/vim-colorschemes.git",
        &flazz_dir,
        force,
    )?;
    let flazz_colors = flazz_dir.join("colors");
    let mut count = 0;
    if flazz_colors.exists() {
        count += copy_vim_files(&flazz_colors, &data_dir)?;
    }
    println!("  Copied {count} .vim files from flazz/vim-colorschemes");

    // Source 2: vim/colorschemes (official, overwrites on conflict = takes precedence)
    let official_dir = tmp_base.join("official");
    clone_repo(
        "https://github.com/vim/colorschemes.git",
        &official_dir,
        force,
    )?;
    let official_colors = official_dir.join("colors");
    let mut official_count = 0;
    if official_colors.exists() {
        official_count = copy_vim_files(&official_colors, &data_dir)?;
    }
    println!("  Copied {official_count} .vim files from vim/colorschemes (takes precedence)");

    // Cleanup
    if tmp_base.exists() {
        let _ = fs::remove_dir_all(&tmp_base);
    }

    let total = fs::read_dir(&data_dir)
        .map(|rd| {
            rd.filter(|e| {
                e.as_ref()
                    .is_ok_and(|e| e.path().extension().and_then(|x| x.to_str()) == Some("vim"))
            })
            .count()
        })
        .unwrap_or(0);

    println!("Fetched {total} vim colorschemes to data/vim/");
    Ok(())
}

fn clone_repo(url: &str, dest: &std::path::Path, force: bool) -> Result<()> {
    if dest.exists() && !force {
        return Ok(());
    }
    if dest.exists() {
        fs::remove_dir_all(dest).with_context(|| format!("removing {}", dest.display()))?;
    }

    println!("  Cloning {url}...");
    let status = Command::new("git")
        .args(["clone", "--depth", "1", url])
        .arg(dest)
        .status()
        .with_context(|| format!("cloning {url}"))?;

    if !status.success() {
        anyhow::bail!("git clone failed for {url}");
    }
    Ok(())
}

fn copy_vim_files(src_dir: &std::path::Path, dest_dir: &std::path::Path) -> Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(src_dir).with_context(|| format!("reading {}", src_dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("vim") {
            continue;
        }
        let file_name = path.file_name().expect("entry has filename");
        let dest = dest_dir.join(file_name);
        fs::copy(&path, &dest).with_context(|| format!("copying {}", path.display()))?;
        count += 1;
    }
    Ok(count)
}
