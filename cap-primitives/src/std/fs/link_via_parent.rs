use crate::fs::{link_unchecked, open_parent, FollowSymlinks, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `link` by `open`ing up the parent component of the path and then
/// calling `linkat` on the last component.
pub(crate) fn link_via_parent(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut old_start = MaybeOwnedFile::Borrowed(old_start);
    let mut new_start = MaybeOwnedFile::Borrowed(new_start);

    let old_basename = match open_parent(&mut old_start, old_path, &mut symlink_count)? {
        // `link` on `..` fails with `EPERM`.
        None => return is_directory(),
        Some(basename) => basename,
    };
    let new_basename = match open_parent(&mut new_start, new_path, &mut symlink_count)? {
        // `link` on `..` fails with `EEXIST`.
        None => return already_exists(),
        Some(basename) => basename,
    };

    link_unchecked(
        old_start.as_file(),
        old_basename,
        new_start.as_file(),
        new_basename,
        FollowSymlinks::No,
    )
}

#[cold]
fn is_directory() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "directories cannot have hard links",
    ))
}

#[cold]
fn already_exists() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "link destination already exists",
    ))
}
