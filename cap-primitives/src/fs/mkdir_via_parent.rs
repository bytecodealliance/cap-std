use crate::fs::{mkdir_unchecked, open_parent, strip_dir_suffix, DirOptions, MaybeOwnedFile};
use std::{fs, io, path::Path};

/// Implement `mkdir` by `open`ing up the parent component of the path and then
/// calling `mkdir_unchecked` on the last component.
pub(crate) fn mkdir_via_parent(
    start: &fs::File,
    path: &Path,
    options: &DirOptions,
) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    // As a special case, `mkdir` ignores a trailing slash rather than treating
    // it as equivalent to a trailing slash-dot, so strip any trailing slashes.
    let path = strip_dir_suffix(path);

    let (dir, basename) = open_parent(start, &path)?;

    mkdir_unchecked(&dir, basename.as_ref(), options)
}
