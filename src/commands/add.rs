use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::Path;

pub fn execute(files: &[String]) -> Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        println!("Not a git repository");
        return Ok(());
    }

    let index_path = git_dir.join("index");

    let mut index_entries = Vec::new();
    if index_path.exists() {
        if let Ok(bytes) = fs::read(&index_path) {
            let content = String::from_utf8_lossy(&bytes);

            for line in content.lines() {
                index_entries.push(line.to_string());
            }
        }
    }

    for file_name in files {
        let file_path = Path::new(file_name);

        if !file_path.exists() {
            println!("warning: {} did not match any files", file_name);
            continue;
        }

        match fs::read(file_path) {
            Ok(content) => {
                let hash = calculate_hash(&content);

                let object_dir = git_dir.join("objects").join(&hash[..2]);
                fs::create_dir_all(&object_dir)?;

                let object_path = object_dir.join(&hash[2..]);
                fs::write(&object_path, &content)?;

                index_entries.push(format!("{} {}", hash, file_path.display()));

                println!("Added '{}' to index", file_path.display());
            }
            Err(e) => {
                println!("warning: could not read file '{}': {}", file_name, e);
            }
        }
    }

    let index_content = index_entries.join("\n") + "\n";
    fs::write(&index_path, index_content)?;

    Ok(())
}

fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(data);
    format!("{:016x}", hasher.finish())
}
