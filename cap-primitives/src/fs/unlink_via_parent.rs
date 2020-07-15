use crate::fs::{open_parent, unlink_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `unlink` by `open`ing up the parent component of the path and then
/// calling `unlink_unchecked` on the last component.
pub(crate) fn unlink_via_parent(start: &fs::File, path: &Path) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut start = MaybeOwnedFile::borrowed(start);

    let basename = open_parent(&mut start, path, &mut symlink_count)?;

    unlink_unchecked(&start, basename.as_ref())
}
