use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::collections::VecDeque;

struct Arguments {
    path: PathBuf,
}

fn main() {
    let args = parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let size = calculate_size(&args.path).unwrap_or_else(|e| {
        eprintln!("Error calculating size for {}: {}", args.path.display(), e);
        std::process::exit(1);
    });

    println!("Path: {}, size: {} bytes", args.path.display(), size);
}

fn parse_arguments() -> Result<Arguments, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(format!("Usage: {} <path>", args[0]));
    }

    match fs::canonicalize(Path::new(&args[1])) {
        Ok(path) => Ok(Arguments { path }),
        Err(e) => Err(format!("Failed to resolve path: {}", e)),
    }
}

fn calculate_size(starting_path: &Path) -> io::Result<u64> {
    let mut total_size = 0;
    let mut dirs_to_visit = VecDeque::new();
    dirs_to_visit.push_back(starting_path.to_path_buf());

    while let Some(path) = dirs_to_visit.pop_front() {
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(e) => {
                eprintln!("Error reading {}: {}", path.display(), e);
                continue;
            }
        };

        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            let entries = match fs::read_dir(&path) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Error reading directory {}: {}", path.display(), e);
                    continue;
                }
            };
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(e) => {
                        eprintln!("Error processing entry: {}", e);
                        continue;
                    }
                };

                let entry_metadata = match entry.metadata() {
                    Ok(metadata) => metadata,
                    Err(e) => {
                        eprintln!("Error reading metadata for {}: {}", entry.path().display(), e);
                        continue;
                    }
                };

                if entry_metadata.file_type().is_symlink() {
                    continue;
                }

                if entry_metadata.is_file() {
                    total_size += entry_metadata.len();
                } else if entry_metadata.is_dir() {
                    dirs_to_visit.push_back(entry.path().to_path_buf());
                }
            }
        }
    }

    Ok(total_size)
}

