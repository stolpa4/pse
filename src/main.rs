use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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

fn calculate_size(path: &Path) -> std::io::Result<u64> {
    let mut total_size = 0;

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        if metadata.is_file() {
            total_size += metadata.len();
        }
    }

    Ok(total_size)
}
