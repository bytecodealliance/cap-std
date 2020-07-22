use super::get_path::{concatenate_or_return_absolute, get_path};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// *Unsandboxed* function similar to `readlink`, but which does not perform sandboxing.
pub(crate) fn readlink_unchecked(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return fs::read_link(path);
    }
    let full_path = concatenate_or_return_absolute(start, path)?;
    fs::read_link(full_path)
}
