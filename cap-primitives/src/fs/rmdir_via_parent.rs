use crate::fs::{open_parent, rmdir_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `rmdir` by `open`ing up the parent component of the path and then
/// calling `rmdir_unchecked` on the last component.
pub(crate) fn rmdir_via_parent(start: &fs::File, path: &Path) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);

    let basename = open_parent(&mut start, path, &mut symlink_count)?;

    rmdir_unchecked(start.as_ref(), basename.as_ref())
}
