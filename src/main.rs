use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

struct Arguments {
    path: PathBuf,
}

fn main() {
    let args = match parse_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let size = calculate_size(&args.path).unwrap_or_else(|e| {
        eprintln!("Error calculating size: {}", e);
        0
    });

    println!("Path: {}, size: {}", args.path.display(), size);
    println!("GoodBye!");
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
        let mut total_size = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            total_size += calculate_size(&path)?;
        }
        return Ok(total_size);
    }

    // If not a file or directory, return 0
    Ok(0)
}
