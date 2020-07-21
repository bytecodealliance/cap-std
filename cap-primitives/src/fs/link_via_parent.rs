use crate::fs::{link_unchecked, open_parent, FollowSymlinks, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `link` by `open`ing up the parent component of the path and then
/// calling `link_unchecked` on the last component.
pub(crate) fn link_via_parent(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut old_start = MaybeOwnedFile::borrowed(old_start);
    let mut new_start = MaybeOwnedFile::borrowed(new_start);

    let old_basename = open_parent(&mut old_start, old_path, &mut symlink_count)?;
    let new_basename = open_parent(&mut new_start, new_path, &mut symlink_count)?;

    link_unchecked(
        &old_start,
        old_basename.as_ref(),
        &new_start,
        new_basename.as_ref(),
        FollowSymlinks::No,
    )
}
