use crate::fs::{open_manually, MaybeOwnedFile, OpenOptions};
use std::{io, path::Path};

/// The primary purpose of this function is to open the "parent" of `path`. `start`
/// is updated to hold the newly opened file descriptor, and the basename of `path`
/// is returned as `Ok(Some(basename))`. Note that the basename may still refer to
/// a symbolic link.
///
/// If `path` ends with `..`, return `Ok(None)`.
///
/// If `path` is absolute, return an error instead.
pub(crate) fn open_parent<'path>(
    start: &mut MaybeOwnedFile,
    path: &'path Path,
    symlink_count: &mut u8,
) -> io::Result<Option<&'path Path>> {
    let parent_path = match path.parent() {
        None => return Err(escape_attempt()),
        Some(parent_path) => parent_path,
    };

    let parent = open_manually(
        start.as_file(),
        parent_path,
        OpenOptions::new().read(true),
        symlink_count,
        None,
    )?;

    *start = MaybeOwnedFile::Owned(parent);

    Ok(path.file_name().map(AsRef::as_ref))
}

#[cold]
fn escape_attempt() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    )
}
