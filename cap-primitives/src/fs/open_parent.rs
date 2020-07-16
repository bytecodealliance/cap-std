use crate::fs::{open_manually, MaybeOwnedFile, OpenOptions};
use std::{
    ffi::OsStr,
    io,
    path::{Component, Path},
};

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
) -> io::Result<Option<&'path OsStr>> {
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

    start.descend_to(parent);

    // This would use `path.file_name()`, except that returns `None` on `.`. We
    // want to see the `.` so that `None` can always mean `..`.
    let file_name = path.components().next_back().and_then(|p| match p {
        Component::Normal(p) => Some(p),
        Component::CurDir => Some(Component::CurDir.as_os_str()),
        _ => None,
    });

    Ok(file_name)
}

#[cold]
fn escape_attempt() -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        "a path led outside of the filesystem",
    )
}
