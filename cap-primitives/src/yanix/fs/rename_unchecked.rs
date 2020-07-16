use std::{fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::renameat;

/// *Unsandboxed* function similar to `rename`, but which does not perform sandboxing.
pub(crate) fn rename_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    unsafe {
        renameat(
            old_start.as_raw_fd(),
            old_path,
            new_start.as_raw_fd(),
            new_path,
        )
    }
}
