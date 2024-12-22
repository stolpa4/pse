mod cli;

use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

struct Arguments {
    path: PathBuf,
}

fn main() {
    let args = parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let size = calculate_size(&args.path);

    println!("Path: {}, size: {} bytes", args.path.display(), size);
}

fn parse_arguments() -> Result<Arguments, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!(
            "Usage: {} <path> [mode(plain/content/recursive, default - plain)]",
            args[0]
        ));
    }

    match fs::canonicalize(Path::new(&args[1])) {
        Ok(path) => Ok(Arguments { path }),
        Err(e) => Err(format!("Failed to resolve path: {}", e)),
    }
}

fn calculate_size(starting_path: &Path) -> u64 {
    let mut total_size = 0;
    let mut dirs_to_visit = VecDeque::new();
    dirs_to_visit.push_back(starting_path.to_path_buf());

    while let Some(path) = dirs_to_visit.pop_front() {
        let (size, subdirs) = process_path(&path);
        total_size += size;
        dirs_to_visit.extend(subdirs);
    }

    total_size
}

#[inline(always)]
fn process_path(path: &Path) -> (u64, Vec<PathBuf>) {
    let mut size = 0;
    let mut subdirs = Vec::new();

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return (0, subdirs),
    };

    if metadata.is_file() {
        return (metadata.len(), subdirs);
    }

    if !metadata.is_dir() {
        return (0, subdirs);
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            if entry_metadata.file_type().is_symlink() {
                continue;
            }

            if entry_metadata.is_file() {
                size += entry_metadata.len();
            } else if entry_metadata.is_dir() {
                subdirs.push(entry.path());
            }
        }
    }

    (size, subdirs)
}
