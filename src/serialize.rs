use std::path::Path;
use crate::proc::FsTree;
use std::io::{self, ErrorKind, Result};
use std::fs;

pub fn serialize_fs_tree<P: AsRef<Path>>(fs_tree: &FsTree, target_path: P, human_readable: bool) -> Result<()> {
    let path: &Path = target_path.as_ref();
    _create_parent(path)?;

    Ok(())
}

#[inline(always)]
fn _create_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            if error.kind() != ErrorKind::AlreadyExists {
                return Err(error);
            }
        }
    }

    Ok(())
}
