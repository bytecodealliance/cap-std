use posish::fs::{unlinkat, AtFlags};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `remove_file`, but which does not perform
/// sandboxing.
pub(crate) fn remove_file_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    unlinkat(start, path, AtFlags::empty())
}
