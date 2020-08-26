//! This defines `set_times`, the primary entrypoint to sandboxed
//! filesystem times modification.
//!
//! TODO: `check_set_times` etc.

#[cfg(not(windows))]
use crate::fs::set_times_nofollow_impl;
use crate::fs::{set_file_times_impl, set_times_impl, FileTimeSpec, FollowSymlinks};
use std::{fs, io, path::Path};

/// Perform a `utimensat`-like operation, ensuring that the resolution of the path
/// never escapes the directory tree rooted at `start`.
#[inline]
pub fn set_times(
    start: &fs::File,
    path: &Path,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
    follow: FollowSymlinks,
) -> io::Result<()> {
    set_times_impl(start, path, atime, mtime, follow)
}

/// Like `set_times`, but never follows symlinks.
#[inline]
#[cfg(not(windows))]
pub(crate) fn set_times_nofollow(
    start: &fs::File,
    path: &Path,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
) -> io::Result<()> {
    set_times_nofollow_impl(start, path, atime, mtime)
}

/// Perform a `futimens`-like operation.
#[inline]
pub fn set_file_times(
    file: &fs::File,
    atime: Option<FileTimeSpec>,
    mtime: Option<FileTimeSpec>,
) -> io::Result<()> {
    set_file_times_impl(file, atime, mtime)
}
