use super::get_path::concatenate_or_return_absolute;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `rename`, but which does not perform sandboxing.
pub(crate) fn rename_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let old_full_path = concatenate_or_return_absolute(old_start, old_path)?;
    let new_full_path = concatenate_or_return_absolute(new_start, new_path)?;
    fs::rename(old_full_path, new_full_path)
}
