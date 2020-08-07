use crate::fs::{read_dir_unchecked, remove_open_dir, rmdir, stat, unlink, FollowSymlinks};
use std::{
    fs, io,
    path::{Component, Path},
};

pub(crate) fn remove_dir_all_impl(start: &fs::File, path: &Path) -> io::Result<()> {
    // Code adapted from `remove_dir_all` in Rust's src/libstd/sys_common/fs.rs
    // at revision 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.
    let filetype = stat(start, path, FollowSymlinks::No)?.file_type();
    if filetype.is_symlink() {
        unlink(start, path)
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
    // src/libstd/sys_common/fs.rs at revision
    // 7e11379f3b4c376fbb9a6c4d44f3286ccc28d149.
    for child in read_dir_unchecked(start, path)? {
        let child = child?;
        if child.file_type()?.is_dir() {
            let path = path.join(child.file_name());
            remove_dir_all_recursive(start, &path)?;
            rmdir(start, &path)?;
        } else {
            child.remove_file()?;
        }
    }
    Ok(())
}
