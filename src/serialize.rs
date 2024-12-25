use crate::proc::{FsEntry, FsTree};
use crate::utils::size_to_label;
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json;
use std::fs::File;
use std::io;

impl Serialize for FsEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
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
                state.serialize_entry("contents", &directory.content)?;
                state.end()
            }
        }
    }
}

pub fn serialize_fs_tree(fs_tree: &FsTree) -> Result<(), io::Error> {
    let file_path = "fs_tree.json";
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, fs_tree)?;
    println!("JSON data was saved to {}", file_path);
    Ok(())
}
