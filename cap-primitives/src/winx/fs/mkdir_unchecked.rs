use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    unimplemented!("mkdir_unchecked")
}
