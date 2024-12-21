use rayon::prelude::*;
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

    let size = calculate_size(&args.path);

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

fn calculate_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge()
        .filter_map(|entry| {
            entry
                .metadata()
                .ok()
                .filter(|meta| meta.is_file())
                .map(|meta| meta.len())
        })
        .sum()
}
