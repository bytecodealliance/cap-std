use crate::fs::{open_parent, unlink_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `unlink` by `open`ing up the parent component of the path and then
/// calling `unlinkat` on the last component.
pub(crate) fn unlink_via_parent(start: &fs::File, path: &Path) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);

    let basename = match open_parent(&mut start, path, &mut symlink_count)? {
        // `unlink` on `..` fails with `EISDIR`.
        None => return is_directory(),
        Some(basename) => basename,
    };

    unlink_unchecked(start.as_file(), basename)
}

fn is_directory() -> io::Result<()> {
    Err(io::Error::new(io::ErrorKind::Other, "is a directory"))
}
