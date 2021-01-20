use super::get_path::concatenate_or_return_absolute;
use std::{fs, io, path::Path};

pub(crate) fn copy_impl(
    from_start: &fs::File,
    from_path: &Path,
    to_start: &fs::File,
    to_path: &Path,
) -> io::Result<u64> {
    let from_full_path = concatenate_or_return_absolute(from_start, from_path)?;
    let to_full_path = concatenate_or_return_absolute(to_start, to_path)?;
    fs::copy(from_full_path, to_full_path)
}
