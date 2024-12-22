use std::collections::{VecDeque, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

struct File {
    path: String,
    size: u64,
}

struct Directory {
    path: String,
    size: u64,
    content: HashMap<String, FsEntry>,
}

enum FsEntry {
    File(File),
    Directory(Directory),
}

type FsTree = HashMap<String, FsEntry>;

pub fn calculate_bulk_size(path: &Path) -> u64 {
    let mut total_size = 0;
    let mut dirs_to_visit = VecDeque::new();
    dirs_to_visit.push_back(path.to_path_buf());

    while let Some(path) = dirs_to_visit.pop_front() {
        let (size, subdirs) = _cbs_process_path(&path);
        total_size += size;
        dirs_to_visit.extend(subdirs);
    }

    total_size
}

#[inline(always)]
fn _cbs_process_path(path: &Path) -> (u64, Vec<PathBuf>) {
    let mut size = 0;
    let mut subdirs = Vec::new();

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return (0, subdirs),
    };

    if metadata.is_file() {
        return (metadata.len(), subdirs);
    }

    if !metadata.is_dir() {
        return (0, subdirs);
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let entry_metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            if entry_metadata.file_type().is_symlink() {
                continue;
            }

            if entry_metadata.is_file() {
                size += entry_metadata.len();
            } else if entry_metadata.is_dir() {
                subdirs.push(entry.path());
            }
        }
    }

    (size, subdirs)
}
