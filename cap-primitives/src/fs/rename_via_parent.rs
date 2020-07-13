use crate::fs::{errors, open_parent, rename_unchecked, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `rename` by `open`ing up the parent component of the path and then
/// calling `renameat` on the last component.
pub fn rename_via_parent(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let mut symlink_count = 0;
    let mut old_start = MaybeOwnedFile::borrowed(old_start);
    let mut new_start = MaybeOwnedFile::borrowed(new_start);

    let old_basename = match open_parent(&mut old_start, old_path, &mut symlink_count)? {
        // `rename` on `..` fails since the path is in use.
        None => return Err(errors::rename_path_in_use()),
        Some(old_basename) => old_basename,
    };

    let new_basename = match open_parent(&mut new_start, new_path, &mut symlink_count)? {
        // `rename` on `..` fails since the path is in use.
        None => return Err(errors::rename_path_in_use()),
        Some(new_basename) => new_basename,
    };

    rename_unchecked(
        old_start.as_file(),
        old_basename.as_ref(),
        new_start.as_file(),
        new_basename.as_ref(),
    )
}
