use crate::utils;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct Arguments {
    pub path: PathBuf,
    pub out_path: PathBuf,
    pub minsize: u64,
}

pub fn parse_arguments() -> io::Result<Arguments> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 || args[1].starts_with("-") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "PSE: Getting the recursive sizes of a fileitem\n\
                 Usage: {} <path> [<out_json_path>]\n\
                 Default out_json_path: $(pwd)/fs_tree.json\n\
                 ENV VARIABLES:\n\
                 |* PSE_MINSIZE - fileitem minimum size threshold\n\
                 |    accepted formats:\n\
                 |      '123123 bytes' - size in bytes\n\
                 |      '100 KB' - size in kilobytes\n\
                 |      '10 MB' - size in megabytes\n\
                 |      '1 GB' - size in gigabytes\n\
                 |      '1 TB' - size in terabytes\n\
                 |    default value: 0 bytes",
                args[0]
            ),
        ));
    }

    Ok(Arguments {
        path: parse_input_path(&args[1])?,
        out_path: parse_output_path(if args.len() >= 3 {
            &args[2]
        } else {
            "fs_tree.json"
        })?,
        minsize: parse_minsize(),
    })
}

#[inline(always)]
fn parse_input_path(path_arg: &str) -> io::Result<PathBuf> {
    fs::canonicalize(Path::new(path_arg))
}

#[inline(always)]
fn parse_output_path(path_arg: &str) -> io::Result<PathBuf> {
    let path = Path::new(path_arg);

    // NOTE: I need to do it by hand, as canonicalize will raise a error if the path does not exist
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let current_dir = env::current_dir()?;
        Ok(current_dir.join(path))
    }
}

fn parse_minsize() -> u64 {
    match env::var("PSE_MINSIZE") {
        Ok(value) => utils::label_to_size(&value).unwrap_or_else(|_| {
            eprintln!("Invalid PSE_MINSIZE format, defaulting to 0 bytes");
            0
        }),
        Err(_) => 0,
    }
}
