use std::env;

use walkdir::{DirEntry, WalkDir};

struct ProgArgs {
    depth: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_args = handle_program_arguments(args);
    let path = env::current_dir().unwrap_or_default();

    for entry in WalkDir::new(&path)
        .sort_by_file_name()
        .max_depth(prog_args.depth)
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

fn handle_program_arguments(args: Vec<String>) -> ProgArgs {
    let args_length = args.len();
    let mut depth: usize = 3;
    if args_length > 2 {
        panic!("Invalid number of arguments");
    } else if args_length == 2 {
        depth = args[1].parse().expect("Invalid argument type");
    }
    ProgArgs { depth }
}
