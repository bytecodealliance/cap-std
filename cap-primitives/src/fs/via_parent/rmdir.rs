use super::open_parent;
use crate::fs::{rmdir_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `rmdir` by `open`ing up the parent component of the path and then
/// calling `rmdir_unchecked` on the last component.
pub(crate) fn rmdir(start: &fs::File, path: &Path) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    rmdir_unchecked(&dir, basename.as_ref())
}
