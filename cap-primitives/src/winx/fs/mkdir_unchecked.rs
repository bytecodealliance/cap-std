use super::get_path::concatenate_or_return_absolute;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    let out_path = concatenate_or_return_absolute(start, path)?;
    fs::create_dir(out_path)
}