use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn execute(files: &[String]) -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        println!("Not a git repository");
        return Ok(());
    }

    let index_path = git_dir.join("Index");
    if !index_path.exists() {
        fs::write(&index_path, "")?;
    }

    for file_name in files {
        let file_path = Path::new(file_name);

        if !file_path.exists() {
            println!("warning: {} did not match any files", file_name);
            continue;
        }
        //we'll just store the file name in the index instead of hashing and storing
        let mut file = fs::File::open(file_path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let hash = calculate_hash(&content);

        let object_dir = git_dir.join("objects").join(&hash[..2]);
        fs::create_dir_all(&object_dir)?;

        //store the files content in objs directory
        let object_path = object_dir.join(&hash[..2]);
        fs::write(&object_path, &content)?;

        println!("Added '{}' to index", file_path.display());
    }

    Ok(())
}

fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
