use std::{ffi::OsStr, fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::{unlinkat, AtFlag};

/// *Unsandboxed* function similar to `unlink`, but which does not perform sandboxing.
pub(crate) fn unlink_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    // POSIX's `unlinkat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    unsafe { unlinkat(start.as_raw_fd(), path, AtFlag::empty()) }
}
