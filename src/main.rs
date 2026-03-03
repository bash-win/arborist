use clap::Parser;
use ignore::{DirEntry, WalkBuilder};
use std::env;

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
    let cwd = env::current_dir().unwrap_or_default();

    for entry in WalkBuilder::new(&cwd).max_depth(Some(args.depth)).build() {
        let entry: DirEntry = entry.expect("Cannot parse the file/directory");
        let full_path = entry.path();

        match full_path.strip_prefix(&cwd) {
            Ok(relative_path) => println!("{}", relative_path.display()),
            Err(_) => println!("{}", full_path.display()),
        }
    }
}
