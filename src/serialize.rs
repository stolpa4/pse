use crate::proc::FsTree;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn serialize_fs_tree<P: AsRef<Path>>(
    fs_tree: &FsTree,
    target_path: P,
    human_readable: bool,
) -> io::Result<()> {
    let path: &Path = target_path.as_ref();
    _create_parent(path)?;

    let file = fs::File::create(path)?;
    let mut writer = io::BufWriter::new(file);

    let lines = vec![
        "Hello, world!",
        "This is a line with UTF-8 characters: ü, ñ, à, 漢字",
        "Rust programming.",
    ];

    for line in lines {
        // Write each line followed by a newline character
        writeln!(writer, "{}", line)?;
    }

    // By this point, all writes have been buffered, ensure they're flushed to the file
    writer.flush()?;

    Ok(())
}

#[inline(always)]
fn _create_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            if error.kind() != io::ErrorKind::AlreadyExists {
                return Err(error);
            }
        }
    }

    Ok(())
}
