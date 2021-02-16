use super::get_path::concatenate_or_return_absolute;
use crate::fs::errors;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `symlink_file`, but which does not perform sandboxing.
pub(crate) fn symlink_file_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let (new_full_path, enforce_dir) = concatenate_or_return_absolute(new_start, new_path)?;
    if enforce_dir {
        return Err(errors::trailing_slash());
    }
    std::os::windows::fs::symlink_file(old_path, new_full_path)
}

/// *Unsandboxed* function similar to `symlink_dir`, but which does not perform sandboxing.
pub(crate) fn symlink_dir_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    let (new_full_path, _enforce_dir) = concatenate_or_return_absolute(new_start, new_path)?;
    std::os::windows::fs::symlink_dir(old_path, new_full_path)
}
