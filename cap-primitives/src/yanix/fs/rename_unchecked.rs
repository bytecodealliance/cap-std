use std::{ffi::OsStr, fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::renameat;

/// *Unsandboxed* function similar to `rename`, but which does not perform sandboxing.
pub(crate) fn rename_unchecked(
    old_start: &fs::File,
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    // POSIX's `renameat` with an empty path returns `ENOENT`, so use "." instead.
    let old_path = if old_path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        old_path.as_ref()
    };
    let new_path = if new_path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        new_path.as_ref()
    };

    unsafe {
        renameat(
            old_start.as_raw_fd(),
            old_path,
            new_start.as_raw_fd(),
            new_path,
        )
    }
}
