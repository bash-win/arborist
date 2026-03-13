# Arborist

A command-line tool that generates directory tree visualizations. Built in Rust.

## Output Example

```
Total:
Directories: 13 | Files: 11
.
├── 📁 src/
│   └── 🦀 main.rs
├── 🔒 Cargo.lock
├── 📦 Cargo.toml
├── 📁 target/
│   ├── CACHEDIR.TAG
│   ├── 📁 debug/
│   │   ├── 📁 examples/
│   │   ├── 📁 deps/
│   │   ├── arborist.d
│   │   ├── 📁 build/
│   │   ├── arborist
│   │   └── 📁 incremental/
│   ├── 📁 release/
│   │   ├── 📁 examples/
│   │   ├── 📁 deps/
│   │   ├── arborist.d
│   │   ├── 📁 build/
│   │   ├── arborist
│   │   └── 📁 incremental/
│   └── 📁 flycheck0/
│       ├── stderr
│       └── stdout
└── 📄 README.md
```


## Installation

Clone the repository and build with Cargo:

```
git clone https://github.com/bash-win/arborist.git
cd arborist
cargo build --release
```

The binary will be at `target/release/arborist`.

## Usage

Run in any directory:

```
arborist
```

### Options

| Flag | Description | Default |
|------|-------------|---------|
| `-d, --depth <N>` | Maximum recursion depth | 3 |
| `-f, --file <PATH>` | Save output to a file | — |
| `-i, --ignore` | Respect `.gitignore` rules | false |
| `-s, --stats` | Print file and directory count summary | false |
| `-c, --comments <COMMENTS_PATH>` | Adds description comments against file names | - |
| `-I, --icons` | Enables file type icons | false |
| `--icon-file` | Custom icon mapping file to override defaults | _ |

### Comment File Format

Arborist has a feature that allows one to add comments to particular files, explaining their contents. The file needs to have lines in the following format:

```
file_name > description
```
Each file_name and it's corresponding description must be specified on a new line.

### Examples

Limit tree depth to 2 levels:

```
arborist --depth 2
```

Save output to a file:

```
arborist --file tree.txt
```

Respect `.gitignore` rules:

```
arborist --ignore
```

## Dependencies

- [clap](https://crates.io/crates/clap) — argument parsing
- [ignore](https://crates.io/crates/ignore) — directory traversal with `.gitignore` support

## License

MIT
