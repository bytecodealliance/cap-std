use super::get_path::concatenate_or_return_absolute;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `remove_dir`, but which does not perform
/// sandboxing.
pub(crate) fn remove_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    let (full_path, _enforce_dir) = concatenate_or_return_absolute(start, path)?;
    fs::remove_dir(full_path)
}
