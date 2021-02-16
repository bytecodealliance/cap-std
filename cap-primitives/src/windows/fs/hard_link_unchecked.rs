use super::get_path::concatenate_or_return_absolute;
use crate::fs::errors;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `hard_link`, but which does not perform sandboxing.
pub(crate) fn hard_link_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let (old_full_path, enforce_dir) = concatenate_or_return_absolute(old_start, old_path)?;

    // Windows rejects trailing slashes in old_path but not new_path.
    if enforce_dir {
        return Err(errors::trailing_slash());
    }

    let (new_full_path, _enforce_dir) = concatenate_or_return_absolute(new_start, new_path)?;

    fs::hard_link(old_full_path, new_full_path)
}
