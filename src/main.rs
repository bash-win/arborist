use clap::Parser;
use ignore::{DirEntry, WalkBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::{env, path::Path};

/// Simple CLI program to generate a directory tree for README files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The depth to which arborist recurses to. Max depth is 50.
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
    name: String,
    full_relative_path: String,
    is_directory: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.depth > 50 {
        return Err("Depth cannot exceed 50".into());
    }

    let cwd = env::current_dir().unwrap_or_default();

    let (tree, directories, files) = build_tree(&args, &cwd);
    let output = render_tree(&tree, "");

    let stats = format!("Total:\nDirectories: {directories} | Files: {files}\n");

    match args.file {
        None => println!("{}.\n{}", stats, output),
        Some(file_path) => {
            let mut output_file = File::create(&file_path)?;
            write!(output_file, "{}.\n{}", stats, output)?;
        }
    }

    Ok(())
}

/// Builds a hashmap which contains the directory tree. The key is the parent path and the values
/// are the files and directories present in the parent.
fn build_tree(args: &Args, cwd: &Path) -> (HashMap<String, Vec<FileInfo>>, usize, usize) {
    let mut tree: HashMap<String, Vec<FileInfo>> = HashMap::new();
    let mut directories = 0;
    let mut files = 0;
    tree.insert(String::new(), Vec::new());

    for entry in WalkBuilder::new(cwd)
        .git_ignore(args.ignore)
        .max_depth(Some(args.depth))
        .build()
    {
        let entry: DirEntry = entry.expect("Cannot parse the file/directory");
        let full_path = entry.path();
        let is_directory = entry
            .file_type()
            .expect("Cannot parse the file type")
            .is_dir();

        if is_directory {
            directories += 1;
        } else {
            files += 1;
        }

        let relative_path = match full_path.strip_prefix(cwd) {
            Ok(p) => p.display().to_string(),
            Err(_) => full_path.display().to_string(),
        };

        if relative_path.is_empty() {
            continue;
        }

        let parent = Path::new(&relative_path)
            .parent()
            .expect("Could not extract the parent path")
            .display()
            .to_string();

        let name = Path::new(&relative_path)
            .file_name()
            .expect("Invalid file path")
            .to_string_lossy()
            .to_string();

        tree.entry(parent).or_default().push(FileInfo {
            name,
            full_relative_path: relative_path,
            is_directory,
        });
    }

    (tree, directories - 1, files)
}

/// Renders the directory using the directory tree hashmap.
fn render_tree(tree: &HashMap<String, Vec<FileInfo>>, key: &str) -> String {
    let mut output = String::new();

    if let Some(children) = tree.get(key) {
        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let connector = if is_last { "└── " } else { "├── " };
            output.push_str(&format!("{}{}", connector, child.name));

            if child.is_directory {
                output.push('/');
            }

            output.push('\n');

            if child.is_directory {
                let subtree = render_tree(tree, &child.full_relative_path);
                let prefix = if is_last { "    " } else { "│   " };
                for line in subtree.lines() {
                    output.push_str(&format!("{}{}\n", prefix, line));
                }
            }
        }
    }

    output
}
