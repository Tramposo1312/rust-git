use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn execute(message: &str) -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        println!("Not a git repository");
        return Ok(());
    }

    let index_path = git_dir.join("index");
    if !index_path.exists() || fs::read_to_string(&index_path)?.trim().is_empty() {
        println!("nothing to commit (create/copy files and use \"rustgit add\" to track)");
        return Ok(());
    }

    let head_content =
        fs::read_to_string(git_dir.join("HEAD")).context("Failed to read HEAD file")?;

    let branch_ref = if head_content.starts_with("ref: ") {
        head_content.trim_start_matches("ref: ").trim()
    } else {
        return Err(anyhow::anyhow!("Detached HEAD state not supported yet"));
    };

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let commit_content = format!(
        "tree: index\n\
        timestamp: {}\n\
        message: {}\n",
        timestamp, message
    );

    let commit_hash = format!("commit_{}", timestamp);
    let branch_path = git_dir.join(branch_ref);

    if let Some(parent) = branch_path.parent() {
        fs::create_dir_all(parent)?;
    }

    //write commit hash to branch ref
    fs::write(&branch_path, &commit_hash)?;

    let commit_dir = git_dir.join("objects").join("commits");
    fs::create_dir_all(&commit_dir)?;
    fs::write(commit_dir.join(&commit_hash), commit_content)?;

    fs::write(&index_path, "")?;

    println!(
        "[{}] {}",
        branch_ref.split('/').last().unwrap_or("unknown"),
        message
    );
    println!(" 1 file changed");

    Ok(())
}
