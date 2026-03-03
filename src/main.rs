use std::env;

use walkdir::{DirEntry, WalkDir};

fn main() {
    let path = env::current_dir().unwrap_or_default();
    for entry in WalkDir::new(path)
        .max_depth(3)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        println!("{}", entry.path().display());
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
