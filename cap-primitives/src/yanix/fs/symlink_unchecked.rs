use std::{fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::symlinkat;

/// *Unsandboxed* function similar to `symlink`, but which does not perform sandboxing.
pub(crate) fn symlink_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    unsafe { symlinkat(old_path, new_start.as_raw_fd(), new_path) }
}
