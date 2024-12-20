use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let path = parse_arguments();
    println!("Specified path: {}", path.display());
    println!("GoodBye!");
}


fn parse_arguments() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    resolve_path(Path::new(&args[1]))
}

fn resolve_path(path: &Path) -> PathBuf {
    match fs::canonicalize(path) {
        Ok(resolved) => resolved,
        Err(e) => {
            eprintln!("Failed to resolve path: {}", e);
            std::process::exit(1);
        },
    }
}
