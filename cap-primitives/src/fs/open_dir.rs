//! This defines `open_dir`, a wrapper around `open` which can be used to open
//! path as a directory.

use crate::fs::{dir_options, open, open_ambient_dir_impl};
use std::{fs, io, path::Path};

/// Open a directory by performing an `openat`-like operation,
/// ensuring that the resolution of the path never escapes
/// the directory tree rooted at `start`.
#[inline]
pub fn open_dir(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open(start, path, &dir_options())
}

/// Open a directory named by a bare path, using the host process' ambient
/// authority.
///
/// # Safety
///
/// This function is not sandboxed and may trivially access any path that the
/// host process has access to.
#[inline]
pub unsafe fn open_ambient_dir(path: &Path) -> io::Result<fs::File> {
    open_ambient_dir_impl(path)
}
