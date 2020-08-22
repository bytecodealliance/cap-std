use super::open_parent;
use crate::fs::{unlink_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `unlink` by `open`ing up the parent component of the path and then
/// calling `unlink_unchecked` on the last component.
pub(crate) fn unlink(start: &fs::File, path: &Path) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    unlink_unchecked(&dir, basename.as_ref())
}
