use anyhow::{Context, Result};
use std::fs;

use crate::codegen::project_root;

pub fn clean(cache: bool, generated: bool) -> Result<()> {
    let root = project_root();

    // Default: if neither flag set, do both
    let (do_cache, do_generated) = if !cache && !generated {
        (true, true)
    } else {
        (cache, generated)
    };

    if do_cache {
        let data_dir = root.join("data");
        if data_dir.exists() {
            fs::remove_dir_all(&data_dir).context("removing data/")?;
            println!("Removed data/");
        } else {
            println!("data/ does not exist, nothing to clean");
        }
    }

    if do_generated {
        for collection in &["base16", "base24", "vim", "emacs"] {
            let dir = root.join("src").join(collection);
            if !dir.exists() {
                continue;
            }
            let mut removed = 0;
            for entry in fs::read_dir(&dir)
                .with_context(|| format!("reading src/{collection}/"))?
            {
                let entry = entry?;
                let path = entry.path();
                // Keep mod.rs, remove everything else
                if path.file_name().and_then(|n| n.to_str()) == Some("mod.rs") {
                    continue;
                }
                if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    fs::remove_file(&path)
                        .with_context(|| format!("removing {}", path.display()))?;
                    removed += 1;
                }
            }
            if removed > 0 {
                println!("Removed {removed} generated files from src/{collection}/");
            }
            // Reset mod.rs to stub
            let mod_path = dir.join("mod.rs");
            let stub = format!(
                "//! {title} color themes.\n\
                 //!\n\
                 //! Enable with the `{collection}` feature flag.\n\n\
                 use crate::Theme;\n\n\
                 /// All themes in the `{collection}` collection.\n\
                 pub static THEMES: &[&Theme] = &[];\n",
                title = match *collection {
                    "base16" => "Base16",
                    "base24" => "Base24",
                    "vim" => "Vim",
                    "emacs" => "Emacs",
                    _ => collection,
                }
            );
            fs::write(&mod_path, stub)
                .with_context(|| format!("resetting {}", mod_path.display()))?;
            println!("Reset src/{collection}/mod.rs to stub");
        }
    }

    Ok(())
}
