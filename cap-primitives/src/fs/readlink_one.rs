use super::{errors::too_many_symlinks, MAX_SYMLINK_EXPANSIONS};
use crate::fs::readlink_unchecked;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

/// This is a wrapper around `readlink_unchecked` which performs a single
/// symlink expansion on a single path component, and which enforces the
/// recursion limit.
pub(crate) fn readlink_one(
    base: &fs::File,
    name: &OsStr,
    symlink_count: &mut u8,
) -> io::Result<PathBuf> {
    let name: &Path = name.as_ref();
    assert!(
        name.as_os_str().is_empty() || name.file_name().is_some(),
        "readlink_one expects a single normal path component, got '{}'",
        name.display()
    );
    assert!(
        name.as_os_str().is_empty() || name.parent().unwrap().as_os_str().is_empty(),
        "readlink_one expects a single normal path component, got '{}'",
        name.display()
    );

    if *symlink_count == MAX_SYMLINK_EXPANSIONS {
        return Err(too_many_symlinks());
    }

    let destination = readlink_unchecked(base, name)?;

    *symlink_count += 1;

    Ok(destination)
}
