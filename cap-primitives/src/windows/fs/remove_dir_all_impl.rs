use crate::fs::{
    read_dir_unchecked, remove_dir, remove_file, remove_open_dir, stat, FollowSymlinks,
};
#[cfg(windows_file_type_ext)]
use std::os::windows::fs::FileTypeExt;
use std::path::{Component, Path};
use std::{fs, io};

pub(crate) fn remove_dir_all_impl(start: &fs::File, path: &Path) -> io::Result<()> {
    // Code derived from `remove_dir_all` in Rust's
    // library/std/src/sys/windows/fs.rs at revision
    // 108e90ca78f052c0c1c49c42a22c85620be19712.
    let filetype = stat(start, path, FollowSymlinks::No)?.file_type();
    if filetype.is_symlink() {
        // On Windows symlinks to files and directories are removed
        // differently. `remove_dir` only deletes dir symlinks and junctions,
        // not file symlinks.
        remove_dir(start, path)
    } else {
        remove_dir_all_recursive(start, path)?;
        remove_dir(start, path)
    }
}

pub(crate) fn remove_open_dir_all_impl(dir: fs::File) -> io::Result<()> {
    remove_dir_all_recursive(&dir, Component::CurDir.as_ref())?;
    remove_open_dir(dir)
}

#[cfg(windows_file_type_ext)]
fn remove_dir_all_recursive(start: &fs::File, path: &Path) -> io::Result<()> {
    // Code derived from `remove_dir_all_recursive` in Rust's
    // library/std/src/sys/windows/fs.rs at revision
    // 108e90ca78f052c0c1c49c42a22c85620be19712.
    for child in read_dir_unchecked(start, path)? {
        let child = child?;
        let child_type = child.file_type()?;
        if child_type.is_dir() {
            let path = path.join(child.file_name());
            remove_dir_all_recursive(start, &path)?;
            remove_dir(start, &path)?;
        } else if child_type.is_symlink_dir() {
            remove_dir(start, &path.join(child.file_name()))?;
        } else {
            remove_file(start, &path.join(child.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(not(windows_file_type_ext))]
fn remove_dir_all_recursive(start: &fs::File, path: &Path) -> io::Result<()> {
    for child in read_dir_unchecked(start, path)? {
        let child = child?;
        let child_type = child.file_type()?;
        if child_type.is_dir() {
            let path = path.join(child.file_name());
            remove_dir_all_recursive(start, &path)?;
            remove_dir(start, &path)?;
        } else {
            match remove_dir(start, &path.join(child.file_name())) {
                Ok(()) => (),
                Err(e) => {
                    if e.raw_os_error() == Some(winapi::shared::winerror::ERROR_DIRECTORY as i32) {
                        remove_file(start, &path.join(child.file_name()))?;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }
    Ok(())
}
