use super::open_parent;
use crate::fs::{set_times_nofollow_unchecked, FileTimeSpec, MaybeOwnedFile};
use std::{fs, io, path::Path};

#[inline]
pub(crate) fn set_times_nofollow(
    start: &fs::File,
    path: &Path,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
) -> io::Result<()> {
    let start = MaybeOwnedFile::borrowed(start);

    let (dir, basename) = open_parent(start, path)?;

    set_times_nofollow_unchecked(&dir, basename.as_ref(), atime, mtime)
}
