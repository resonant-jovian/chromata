use anyhow::{Context, Result, bail};
use std::fs;

use super::clone_tinted_theming;
use crate::codegen::project_root;

pub fn fetch(force: bool) -> Result<()> {
    let project_root = project_root();
    let data_dir = project_root.join("data").join("base16");
    fs::create_dir_all(&data_dir).context("creating data/base16/")?;

    let tmp_dir = clone_tinted_theming(force)?;

    let schemes_dir = tmp_dir.join("base16");
    if !schemes_dir.exists() {
        bail!("No base16/ directory found in cloned schemes repo");
    }

    let mut count = 0;
    for entry in fs::read_dir(&schemes_dir).context("reading schemes/base16/")? {
        let entry = entry.context("reading directory entry")?;
        let path = entry.path();

        let ext = path.extension().and_then(|e| e.to_str());
        if ext != Some("yaml") && ext != Some("yml") {
            continue;
        }

        let file_name = path.file_name().expect("entry has filename");
        let dest = data_dir.join(file_name);
        fs::copy(&path, &dest).with_context(|| format!("copying {}", path.display()))?;
        count += 1;
    }

    println!("Fetched {count} base16 schemes to data/base16/");
    Ok(())
}
