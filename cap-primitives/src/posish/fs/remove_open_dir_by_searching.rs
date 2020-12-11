use crate::fs::{errors, is_root_dir, read_dir_unchecked, Metadata};
use std::{fs, io, path::Component};

/// Delete the directory referenced by the given handle by searching for it in
/// its `..`. This requires search permission in `..`, but that's usually
/// available.
pub(crate) fn remove_open_dir_by_searching(dir: fs::File) -> io::Result<()> {
    let metadata = Metadata::from_file(&dir)?;
    let mut iter = read_dir_unchecked(&dir, Component::ParentDir.as_os_str().as_ref())?;
    while let Some(child) = iter.next() {
        let child = child?;
        if child.is_same_file(&metadata)? {
            return child.remove_dir();
        }
    }

    // We didn't find the directory among its parent's children. Check for the
    // root directory and handle it specially -- removal will probably fail, so
    // we'll get the appropriate error code.
    if is_root_dir(&dir, &iter)? {
        fs::remove_dir(Component::RootDir.as_os_str())
    } else {
        Err(errors::no_such_file_or_directory())
    }
}
