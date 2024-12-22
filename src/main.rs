use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Mode {
    Plain,
    Content,
    Recursive,
}

struct Arguments {
    path: PathBuf,
    mode: Mode,
}

fn main() {
    let args = parse_arguments().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let size = calculate_size(&args.path);

    println!("Mode: {:?}", args.mode);
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

    let path = match fs::canonicalize(Path::new(&args[1])) {
        Ok(path) => path,
        Err(e) => return Err(format!("Failed to resolve path: {}", e)),
    };

    let mode = if args.len() > 2 {
        parse_mode(&args[2])?
    } else {
        Mode::Plain // Default
    };

    Ok(Arguments { path, mode })
}

fn parse_mode(mode_str: &str) -> Result<Mode, String> {
    match mode_str.to_lowercase().as_str() {
        "plain" => Ok(Mode::Plain),
        "content" => Ok(Mode::Content),
        "recursive" => Ok(Mode::Recursive),
        _ => Err(format!(
            "Invalid mode: {}. Use 'plain', 'content', or 'recursive'.",
            mode_str
        )),
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
