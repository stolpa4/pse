use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

pub struct File {
    pub path: String,
    pub size: u64,
}

pub struct Directory {
    pub path: String,
    pub size: u64,
    pub content: HashMap<String, FsEntry>,
}

pub enum FsEntry {
    File(File),
    Directory(Directory),
}

pub type FsTree = HashMap<String, FsEntry>;

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
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return (0, Vec::new()),
    };

    if metadata.is_file() {
        return (metadata.len(), Vec::new());
    }

    if !metadata.is_dir() {
        return (0, Vec::new());
    }

    _cbs_process_dir_path(path)
}

#[inline(always)]
fn _cbs_process_dir_path(path: &Path) -> (u64, Vec<PathBuf>) {
    let mut size = 0;
    let mut subdirs = Vec::new();

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
    let mut fs_tree = FsTree::new();

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
        add_file_to_fs_tree(&mut fs_tree, &path, metadata.len());
    } else if metadata.is_dir() {
        add_dir_to_fs_tree(&mut fs_tree, &path);
    }

    fs_tree
}

#[inline(always)]
fn add_file_to_fs_tree(fs_tree: &mut FsTree, path: &Path, size: u64) {
    fs_tree.insert(
        path.file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string(),
        FsEntry::File(File {
            path: path.to_string_lossy().to_string(),
            size,
        }),
    );
}

#[inline(always)]
fn add_dir_to_fs_tree(fs_tree: &mut FsTree, path: &Path) -> u64 {
    let mut content_fs_tree = FsTree::new();
    let mut dir_full_size = 0;

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
                dir_full_size += entry_metadata.len();
                add_file_to_fs_tree(&mut content_fs_tree, &entry.path(), entry_metadata.len());
            } else if entry_metadata.is_dir() {
                dir_full_size += add_dir_to_fs_tree(&mut content_fs_tree, &entry.path());
            }
        }

        fs_tree.insert(
            path.file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy()
                .to_string(),
            FsEntry::Directory(Directory {
                path: path.to_string_lossy().to_string(),
                size: dir_full_size,
                content: content_fs_tree,
            }),
        );
    }

    dir_full_size
}
