use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct Arguments {
    pub path: PathBuf,
}

pub fn parse_arguments() -> Result<Arguments, String> {
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
