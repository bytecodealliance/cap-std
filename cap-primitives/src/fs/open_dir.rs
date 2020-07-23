//! This defines `open_dir`, a wrapper around `open` which can be used to open
//! path as a directory.

use crate::fs::{dir_options, open};
use std::{fs, io, path::Path};

/// Open a directory by performing an `openat`-like operation,
/// ensuring that the resolution of the path never escapes
/// the directory tree rooted at `start`.
pub fn open_dir(start: &fs::File, path: &Path) -> io::Result<fs::File> {
    open(start, path, &dir_options())
}
