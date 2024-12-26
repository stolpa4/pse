use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct Arguments {
    pub path: PathBuf,
    pub out_path: PathBuf,
}

pub fn parse_arguments() -> io::Result<Arguments> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Usage: {} <path> [<out_json_path>]", args[0]),
        ));
    }

    Ok(Arguments {
        path: _parse_input_path(&args[1])?,
        out_path: _parse_output_path(if args.len() >= 3 {
            &args[2]
        } else {
            "fs_tree.json"
        })?,
    })
}

#[inline(always)]
fn _parse_input_path(path_arg: &str) -> io::Result<PathBuf> {
    fs::canonicalize(Path::new(path_arg))
}

#[inline(always)]
fn _parse_output_path(path_arg: &str) -> io::Result<PathBuf> {
    let path = Path::new(path_arg);

    // NOTE: I need to do it by hand, as canonicalize will raise a error if the path does not exist
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(path))
    }
}
