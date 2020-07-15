use crate::fs::{open_parent, rename_unchecked, strip_dir_suffix, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `rename` by `open`ing up the parent component of the path and then
/// calling `rename_unchecked` on the last component.
pub fn rename_via_parent(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut old_start = MaybeOwnedFile::borrowed(old_start);
    let mut new_start = MaybeOwnedFile::borrowed(new_start);

    // As a special case, `rename` ignores a trailing slash rather than treating
    // it as equivalent to a trailing slash-dot, so strip any trailing slashes.
    let old_path = strip_dir_suffix(old_path);
    let new_path = strip_dir_suffix(new_path);

    let old_basename = open_parent(&mut old_start, old_path, &mut symlink_count)?;
    let new_basename = open_parent(&mut new_start, new_path, &mut symlink_count)?;

    rename_unchecked(
        old_start.as_ref(),
        old_basename.as_ref(),
        new_start.as_ref(),
        new_basename.as_ref(),
    )
}
