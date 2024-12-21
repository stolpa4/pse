use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

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

fn calculate_size(path: &Path) -> io::Result<u64> {
    if path.is_file() {
        return Ok(fs::metadata(path)?.len());
    }

    if path.is_dir() {
        let entries = fs::read_dir(path)?;
        let mut total_size = 0;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            total_size += calculate_size(&path).unwrap_or_else(|e| {
                eprintln!("Error reading {}: {}", path.display(), e);
                0
            });
        }
        Ok(total_size)
    } else {
        Ok(0)
    }
}
