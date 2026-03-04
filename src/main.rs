use clap::Parser;
use ignore::{DirEntry, WalkBuilder};
use std::{env, path::Path};

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

    /// Respect .gitignore rules.
    #[arg(short, long, default_value_t = false)]
    ignore: bool,

    /// Print a summary at the bottom which shows the number of directories and files
    #[arg(short, long, default_value_t = false)]
    stats: bool,
}

#[derive(Debug)]
struct FileInfo {
    relative_path: String,
    depth: usize,
    is_directory: bool,
}

fn main() {
    let args = Args::parse();
    let cwd = env::current_dir().unwrap_or_default();

    let raw_output = get_raw_directory_output(&args, &cwd);
    for item in raw_output {
        println!("{item:?}");
    }
}

fn get_raw_directory_output(args: &Args, cwd: &Path) -> Vec<FileInfo> {
    let mut result: Vec<FileInfo> = Vec::new();

    for entry in WalkBuilder::new(cwd)
        .git_ignore(args.ignore)
        .max_depth(Some(args.depth))
        .build()
    {
        let entry: DirEntry = entry.expect("Cannot parse the file/directory");
        let full_path = entry.path();
        let depth = entry.depth();
        let is_directory = entry.file_type().is_some_and(|f| f.is_dir());

        let relative_path = match full_path.strip_prefix(cwd) {
            Ok(relative_path) => relative_path.display().to_string(),
            Err(_) => full_path.display().to_string(),
        };
        if !relative_path.is_empty() {
            result.push(FileInfo {
                relative_path,
                depth,
                is_directory,
            });
        }
    }

    result
}
