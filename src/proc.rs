use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub struct File {
    pub path: String,
    pub size: u64,
}

pub struct Directory {
    pub path: String,
    pub size: u64,
    pub contents: FsTree,
}

pub enum FsEntry {
    File(File),
    Directory(Directory),
}

impl FsEntry {
    pub fn entry_type(&self) -> &str {
        match self {
            FsEntry::File(_) => "file",
            FsEntry::Directory(_) => "directory",
        }
    }

    pub fn size(&self) -> u64 {
        match self {
            FsEntry::File(file) => file.size,
            FsEntry::Directory(directory) => directory.size,
        }
    }
}

pub type FsTree = Vec<FsEntry>;

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

    sort_fs_tree(&mut fs_tree);

    fs_tree
}

#[inline(always)]
fn add_file_to_fs_tree(fs_tree: &mut FsTree, path: &Path, size: u64) {
    fs_tree.push(
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

        fs_tree.push(
            FsEntry::Directory(Directory {
                path: path.to_string_lossy().to_string(),
                size: dir_full_size,
                contents: content_fs_tree,
            }),
        );
    }

    dir_full_size
}

#[inline(always)]
fn sort_fs_tree(fs_tree: &mut FsTree) {
    fs_tree.sort_by(|a, b| {
        let type_cmp = b.entry_type().cmp(&a.entry_type());
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }
        b.size().cmp(&a.size())
    });

    for entry in fs_tree.iter_mut() {
        if let FsEntry::Directory(ref mut directory) = entry {
            sort_fs_tree(&mut directory.contents);
        }
    }
}
