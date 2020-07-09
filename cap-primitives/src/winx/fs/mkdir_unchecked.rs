use super::get_path;
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    let start_path = get_path::get_path(start)?;
    fs::create_dir(start_path.join(path))
}
