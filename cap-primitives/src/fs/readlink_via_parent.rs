use crate::fs::{open_parent, readlink_unchecked, MaybeOwnedFile};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Implement `readlink` by `open`ing up the parent component of the path and then
/// calling `readlink_unchecked` on the last component.
pub fn readlink_via_parent(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    readlink_unchecked(&dir, basename.as_ref())
}
