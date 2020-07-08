use crate::fs::{open_parent, symlink_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `symlink` by `open`ing up the parent component of the path and then
/// calling `symlinkat` on the last component.
pub fn symlink_via_parent(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut new_start = MaybeOwnedFile::Borrowed(new_start);

    let new_basename = match open_parent(&mut new_start, new_path, &mut symlink_count)? {
        // `symlink` on `..` fails with `EEXIST`.
        None => return already_exists(),
        Some(new_basename) => new_basename,
    };

    symlink_unchecked(old_path, new_start.as_file(), new_basename)
}

fn already_exists() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "directory already exists",
    ))
}
