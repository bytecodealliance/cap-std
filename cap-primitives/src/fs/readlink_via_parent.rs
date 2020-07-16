use crate::fs::{open_parent, readlink_unchecked, MaybeOwnedFile};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Implement `readlink` by `open`ing up the parent component of the path and then
/// calling `readlinkat` on the last component.
pub fn readlink_via_parent(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);

    let basename = open_parent(&mut start, path, &mut symlink_count)?;

    readlink_unchecked(start.as_file(), basename.as_ref())
}
