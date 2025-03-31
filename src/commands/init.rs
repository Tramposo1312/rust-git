use crate::utils;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn execute() -> Result<()> {
    let git_dir = Path::new(".git");

    if git_dir.exists() {
        println!("Repository already initialised.");
        return Ok(());
    }

    fs::create_dir(git_dir)?;
    fs::create_dir(git_dir.join("objects"))?;
    fs::create_dir(git_dir.join("refs"))?;
    fs::create_dir(git_dir.join("refs/heads"))?;

    utils::safe_write_text_file(&git_dir.join("HEAD"), "ref: refs/heads/master\n")?;

    println!("Initialised empty git repository in .git/");
    Ok(())
}
