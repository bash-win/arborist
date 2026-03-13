use clap::Parser;
use ignore::{DirEntry, WalkBuilder};
use std::collections::HashMap;
use std::fs::{File, read_to_string};
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

    /// Print a summary at the top which shows the number of directories and files
    #[arg(short, long, default_value_t = false)]
    stats: bool,

    /// File containing the file names and the description of the files to be displayed.
    #[arg(short, long)]
    comments: Option<String>,
}

#[derive(Debug)]
struct FileInfo {
    name: String,
    full_relative_path: String,
    is_directory: bool,
}

struct TreeLine {
    text: String,
    file_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.depth > 50 {
        return Err("Depth cannot exceed 50".into());
    }

    let cwd = env::current_dir().unwrap_or_default();

    let (tree, directories, files) = build_tree(&args, &cwd);
    let lines = build_tree_lines(&tree, "");
    let comments = args.comments.map(parse_comment_file_into_hashmap);
    let output = format_with_comments(&lines, &comments);

    let stats = match args.stats {
        true => format!("Total:\nDirectories: {directories} | Files: {files}\n"),
        false => String::new(),
    };

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

fn parse_comment_file_into_hashmap(path: String) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for line in read_to_string(path).unwrap_or_default().lines() {
        let Some((file_name, comment)) = line.split_once('>') else {
            continue;
        };
        result.insert(file_name.trim().to_string(), comment.trim().to_string());
    }

    result
}

/// Builds a vector of tree lines with the display text and file name for comment lookup.
fn build_tree_lines(tree: &HashMap<String, Vec<FileInfo>>, key: &str) -> Vec<TreeLine> {
    let mut lines: Vec<TreeLine> = Vec::new();

    if let Some(children) = tree.get(key) {
        for (i, child) in children.iter().enumerate() {
            let is_last = i == children.len() - 1;
            let connector = if is_last { "└── " } else { "├── " };

            let mut text = format!("{}{}", connector, child.name);
            if child.is_directory {
                text.push('/');
            }

            lines.push(TreeLine {
                text,
                file_name: child.name.clone(),
            });

            if child.is_directory {
                let subtree = build_tree_lines(tree, &child.full_relative_path);
                let prefix = if is_last { "    " } else { "│   " };
                for sub_line in subtree {
                    lines.push(TreeLine {
                        text: format!("{}{}", prefix, sub_line.text),
                        file_name: sub_line.file_name,
                    });
                }
            }
        }
    }

    lines
}

/// Appends comments with a fixed gap after each line.
fn format_with_comments(lines: &[TreeLine], comments: &Option<HashMap<String, String>>) -> String {
    let mut output = String::new();
    for line in lines {
        output.push_str(&line.text);

        if let Some(comment) = comments.as_ref().and_then(|c| c.get(&line.file_name)) {
            output.push_str(&format!("    # {}", comment));
        }

        output.push('\n');
    }

    output
}
