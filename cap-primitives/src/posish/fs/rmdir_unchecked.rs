use posish::fs::{unlinkat, AtFlags};
use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `rmdir`, but which does not perform sandboxing.
pub(crate) fn rmdir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    unlinkat(start, path, AtFlags::REMOVEDIR)
}
