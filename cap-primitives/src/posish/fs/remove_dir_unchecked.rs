use posish::fs::{unlinkat, AtFlags};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `remove_dir`, but which does not perform
/// sandboxing.
pub(crate) fn remove_dir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    Ok(unlinkat(start, path, AtFlags::REMOVEDIR)?)
}
