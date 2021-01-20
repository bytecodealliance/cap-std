use super::get_path::concatenate_or_return_absolute;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `remove_file`, but which does not perform
/// sandboxing.
pub(crate) fn remove_file_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    let full_path = concatenate_or_return_absolute(start, path)?;
    fs::remove_file(full_path)
}
