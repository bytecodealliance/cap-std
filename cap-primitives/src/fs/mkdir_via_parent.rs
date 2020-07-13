use crate::fs::{mkdir_unchecked, open_parent, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `mkdir` by `open`ing up the parent component of the path and then
/// calling `mkdirat` on the last component.
pub(crate) fn mkdir_via_parent(start: &fs::File, path: &Path) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);

    let basename = match open_parent(&mut start, path, &mut symlink_count)? {
        // `mkdir` on `..` fails with `EEXIST`.
        None => return already_exists(),
        Some(basename) => basename,
    };

    mkdir_unchecked(start.as_file(), basename.as_ref())
}

fn already_exists() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "directory already exists",
    ))
}
