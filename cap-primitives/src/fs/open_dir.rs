//! This defines `open_dir`, a wrapper around `open` which can be used to open
//! path as a directory.

#[allow(unused_imports)]
use crate::fs::open_unchecked;
use crate::fs::{dir_options, dir_path_options, open, open_ambient_dir_impl};
use std::{fs, io, path::Path};

/// Open a directory by performing an `openat`-like operation,
/// ensuring that the resolution of the path never escapes
/// the directory tree rooted at `start`.
#[inline]
pub fn open_dir(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open(start, path, &dir_options())
}

/// Open a directory by performing an unsandboxed `openat`-like operation.
#[inline]
#[allow(dead_code)]
pub(crate) fn open_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open_unchecked(start, path, &dir_options()).map_err(Into::into)
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

/// Similar to `openat`, but may only be usable for use as the `start`
/// parameter in `cap-primitives` functions.
#[inline]
pub fn open_dir_path(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open(start, path, &dir_path_options())
}
