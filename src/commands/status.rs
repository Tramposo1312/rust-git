use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
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

    let staged_files = get_staged_files(git_dir)?;

    let untracked_files = get_untracked_files()?;

    if staged_files.is_empty() && untracked_files.is_empty() {
        println!("nothing to commit, working tree clean");
    } else {
        if !staged_files.is_empty() {
            println!("\nChanges to be commited:");
            for file in staged_files {
                println!("  new file: {}", file.display());
            }
        }

        if !untracked_files.is_empty() {
            println!("\nUntracked files:");
            for file in untracked_files {
                println!("  {}", file.display());
            }
        }

        if !staged_files.is_empty() {
            println!("\n use \"rustgit commit\" to commit your changes");
        } else {
            println!("\n nothing added to commit but untracked files are present");
        }
    }
    Ok(())
}

fn get_staged_files(git_dir: &Path) -> Result<HashSet<PathBuf>> {
    let mut result = HashSet::new();

    let index_path = git_dir.join("index");
    if index_path.exists() {
        let index_content = fs::read_to_string(index_path)?;

        //very simplified parsing of index file
        for line in index_content.line() {
            if let Some(file_path) = line.split_whitespace().nth(1) {
                result.insert(PathBuf::from(file_path));
            }
        }
    }

    Ok(result)
}

fn get_untracked_files(staged_files: &HashSet<PathBuf>) -> Result<Vec<PathBuf>> {
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

        if staged_files.contains(&path) {
            continue;
        }
        //consider all files untracked
        if path.is_file() {
            result.push(path);
        }
    }

    Ok(result)
}
