use crate::proc::{FsEntry, FsTree};
use crate::utils::size_to_label;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

impl Serialize for FsEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FsEntry::File(ref file) => {
                let mut state = serializer.serialize_map(Some(2))?;
                state.serialize_entry("type", "file")?;
                state.serialize_entry("path", &file.path)?;
                state.serialize_entry("size", &size_to_label(file.size))?;
                state.end()
            }
            FsEntry::Directory(ref directory) => {
                let mut state = serializer.serialize_map(Some(3))?;
                state.serialize_entry("type", "directory")?;
                state.serialize_entry("path", &directory.path)?;
                state.serialize_entry("size", &size_to_label(directory.size))?;
                state.serialize_entry("contents", &directory.contents)?;
                state.end()
            }
        }
    }
}

pub fn serialize_fs_tree<P: AsRef<Path>>(path: P, fs_tree: &FsTree) -> Result<(), io::Error> {
    let path: &Path = path.as_ref();
    _create_parent(path)?;
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, fs_tree)?;
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
