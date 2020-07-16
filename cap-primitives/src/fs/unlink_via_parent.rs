use crate::fs::{open_parent, unlink_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `unlink` by `open`ing up the parent component of the path and then
/// calling `unlinkat` on the last component.
pub(crate) fn unlink_via_parent(start: &fs::File, path: &Path) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);

    let basename = open_parent(&mut start, path, &mut symlink_count)?;

    unlink_unchecked(start.as_file(), basename.as_ref())
}
