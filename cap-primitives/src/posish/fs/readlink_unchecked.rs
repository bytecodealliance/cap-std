use posish::fs::readlinkat;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// *Unsandboxed* function similar to `readlink`, but which does not perform sandboxing.
pub(crate) fn readlink_unchecked(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    readlinkat(start, path).map(Into::into)
}
