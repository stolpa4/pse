use std::collections::{VecDeque, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

enum FsEntryType {
    File,
    Directory,
}

struct File {
    path: String,
    size: u64,
    fs_type: FsEntryType,
}

struct Directory {
    path: String,
    size: u64,
    fs_type: FsEntryType,
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

pub fn build_fs_tree(path: &Path) -> FsTree {
    let mut fs_tree = HashMap::new();

    let path = match path.canonicalize() {
        Ok(m) => m,
        Err(_) => return fs_tree,
    };

    let metadata = match fs::metadata(&path) {
        Ok(m) => m,
        Err(_) => return fs_tree,
    };

    if metadata.file_type().is_symlink() {
        return fs_tree;
    }

    if metadata.is_file() {
        fs_tree.insert(path.to_string_lossy().to_string(), FsEntry::File(File {
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            fs_type: FsEntryType::File,
        }));
    } else if metadata.is_dir() {
        add_dir_to_fs_tree(&mut fs_tree, &path);
    }

    fs_tree
}

fn add_dir_to_fs_tree(fs_tree: &mut FsTree, path: &Path) {
    // TODO
}
