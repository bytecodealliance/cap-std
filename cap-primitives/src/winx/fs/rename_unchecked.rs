use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `rename`, but which does not perform sandboxing.
pub(crate) fn rename_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    todo!("rename_unchecked")
}
