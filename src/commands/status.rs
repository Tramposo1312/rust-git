use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute() -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        println!("Not a git repository");
        return Ok(());
    }
    // Read HEAD
    let head_content =
        fs::read_to_string(git_dir.join("HEAD")).context("Failed to read HEAD file")?;

    let branch_name = if head_content.starts_with("ref: refs/heads/") {
        head_content.trim_start_matches("ref: refs/heads/").trim()
    } else {
        "detached HEAD"
    };

    println!("On branch {}", branch_name);

    let untracked_files = get_untracked_files()?;

    if untracked_files.is_empty() {
        println!("nothing to commit, wokring tree clean");
    } else {
        println!("\nUntracked files:");
        for file in untracked_files {
            println!("  {}", file.display());
        }
        println!("\nnothing added to commit but untracked files present");
    }

    Ok(())
}

fn get_untracked_files() -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        //skip .git and hidden files
        if path
            .file_name()
            .map_or(false, |n| n.to_string_lossy().starts_with("."))
        {
            continue;
        }
        //consider all files untracked
        if path.is_file() {
            result.push(path);
        }
    }

    Ok(result)
}
