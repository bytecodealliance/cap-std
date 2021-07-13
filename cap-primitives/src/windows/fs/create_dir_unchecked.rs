use super::get_path::concatenate_or_return_absolute;
use crate::fs::DirOptions;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `create_dir`, but which does not perform
/// sandboxing.
///
/// Note that Windows doesn't have any extra flags in `DirOptions`, so the
/// `options` parameter is ignored.
pub(crate) fn create_dir_unchecked(
    start: &fs::File,
    path: &Path,
    _options: &DirOptions,
) -> io::Result<()> {
    let out_path = concatenate_or_return_absolute(start, path)?;
    fs::create_dir(out_path)
}
