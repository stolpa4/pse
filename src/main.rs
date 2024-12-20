use std::env;
use std::fs;
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

    println!("Specified path: {}", args.path.display());
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
