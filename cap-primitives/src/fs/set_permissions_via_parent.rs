use crate::fs::{open_parent, set_permissions_unchecked, MaybeOwnedFile, Permissions};
use std::{fs, io, path::Path};

#[inline]
pub(crate) fn set_permissions_via_parent(
    start: &fs::File,
    path: &Path,
    perm: Permissions,
) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, &path)?;

    set_permissions_unchecked(&dir, basename.as_ref(), perm)
}
