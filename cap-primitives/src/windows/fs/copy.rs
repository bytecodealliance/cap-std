use super::get_path::concatenate_or_return_absolute;
use crate::fs::errors;
use std::{fs, io, path::Path};

pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    let (from_full_path, enforce_dir) = concatenate_or_return_absolute(from_start, from_path)?;
    if enforce_dir {
        return Err(errors::trailing_slash());
    }

    let (to_full_path, enforce_dir) = concatenate_or_return_absolute(to_start, to_path)?;
    if enforce_dir {
        return Err(errors::trailing_slash());
    }

    fs::copy(from_full_path, to_full_path)
}
