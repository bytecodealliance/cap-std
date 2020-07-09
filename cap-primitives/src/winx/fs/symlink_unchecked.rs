use std::{fs, io, path::Path};

/// *Unsandboxed* function similar to `symlink_file`, but which does not perform sandboxing.
pub(crate) fn symlink_file_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    todo!("symlink_file_unchecked")
}

/// *Unsandboxed* function similar to `symlink_dir`, but which does not perform sandboxing.
pub(crate) fn symlink_dir_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    todo!("symlink_dir_unchecked")
}
