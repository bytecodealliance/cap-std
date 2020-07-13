use std::{ffi::OsStr, fs, io, path::PathBuf};

const MAX_SYMLINK_EXPANSIONS: u8 = 40;

/// This is a wrapper around `readlink_unchecked` which performs a single
/// symlink expansion on a single path component, and which enforces the
/// recursion limit.
pub(crate) fn readlink_one(
    base: &fs::File,
    name: &OsStr,
    symlink_count: &mut u8,
) -> io::Result<PathBuf> {
    assert!(Path::new(name).parent().is_none());
    assert!(Path::new(name).file_name().is_some());

    todo!("readlink_one")
}
