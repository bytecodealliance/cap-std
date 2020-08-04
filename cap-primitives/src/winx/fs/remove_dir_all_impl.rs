use crate::fs::{read_dir, remove_open_dir, rmdir, stat, unlink, FollowSymlinks};
use std::{
    fs, io,
    path::{Component, Path},
};

pub(crate) fn remove_dir_all_impl(start: &fs::File, path: &Path) -> io::Result<()> {
    // Code adapted from `remove_dir_all` in Rust's src/libstd/sys/windows/fs.rs
    // at revision 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.
    let filetype = stat(start, path, FollowSymlinks::No)?.file_type();
    if filetype.is_symlink() {
        // On Windows symlinks to files and directories are removed differently.
        // rmdir only deletes dir symlinks and junctions, not file symlinks.
        rmdir(start, path)
    } else {
        remove_dir_all_recursive(start, path)?;
        rmdir(start, path)
    }
}

pub(crate) fn remove_open_dir_all_impl(dir: fs::File) -> io::Result<()> {
    remove_dir_all_recursive(&dir, Component::CurDir.as_os_str().as_ref())?;
    remove_open_dir(dir)
}

fn remove_dir_all_recursive(start: &fs::File, path: &Path) -> io::Result<()> {
    // Code adapted from `remove_dir_all_recursive` in Rust's
    // src/libstd/sys/windows/fs.rs at revision
    // 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.
    for child in read_dir(start, path)? {
        let child = child?;
        let child_type = child.file_type()?;
        if child_type.is_dir() {
            let path = path.join(child.file_name());
            remove_dir_all_recursive(start, &path)?;
            rmdir(start, &path)?;
        } else if child_type.is_symlink_dir() {
            rmdir(start, &path.join(child.file_name()))?;
        } else {
            unlink(start, &path.join(child.file_name()))?;
        }
    }
}
