use super::get_path::concatenate_or_return_absolute;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// *Unsandboxed* function similar to `read_link`, but which does not perform sandboxing.
pub(crate) fn read_link_unchecked(
    start: &fs::File,
    path: &Path,
    _reuse: PathBuf,
) -> io::Result<PathBuf> {
    let full_path = concatenate_or_return_absolute(start, path)?;
    fs::read_link(full_path)
}
