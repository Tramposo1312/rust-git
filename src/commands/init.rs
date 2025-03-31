use anyhow::{COntext, Result};
use std::fs;
use std::path::Path;

pub fn execute() -> Result<()> {
    let git_dir = Path::new(".git");

    if git_dir.exists() {
        println!("Repository already initialised.");
        return Ok(());
    }

    //Basic repo struct
    fs::create_dir(git_dir).context("Failed to create .git directory")?;
    fs::create_dir(git_dir.join("objects")).context("Failed to create objects directory")?;
    fs::create_dir(git_dir.join("refs")).context("Failed to create refs directory")?;
    fs::create_dir(git_dir.join("refs/heads")).context("Failed to create refs/heads directory")?;

    //Initial HEAD file
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")
        .context("Failed to create HEAD file")?;

    println!("Initialised empty git repository in .git/");
    Ok(())
}
