use clap::Parser;
use std::env;

use walkdir::{DirEntry, WalkDir};

/// Simple CLI program to generate a directory tree for README files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The depth to which arborist recurses to
    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    /// Saves the output to a file
    #[arg(short, long)]
    file: Option<String>,

    /// Respect .gitignore rules. Automatic behavior. Optionally, provide your own file
    #[arg(short, long)]
    ignore: Option<String>,

    /// Print a summary at the bottom which shows the number of directories and files
    #[arg(short, long, default_value_t = false)]
    stats: bool,
}

fn main() {
    let args = Args::parse();
    let path = env::current_dir().unwrap_or_default();

    for entry in WalkDir::new(&path)
        .sort_by_file_name()
        .max_depth(args.depth)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        println!(
            "{}",
            entry
                .path()
                .display()
                .to_string()
                .replace(path.to_str().expect("Could not convert path to string"), "")
        );
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
