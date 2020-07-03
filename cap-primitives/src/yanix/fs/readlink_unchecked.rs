use std::{
    ffi::OsStr,
    fs, io,
    os::unix::io::AsRawFd,
    path::{Path, PathBuf},
};
use yanix::file::readlinkat;

/// *Unsandboxed* function similar to `readlink`, but which does not perform sandboxing.
pub(crate) fn readlink_unchecked(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    // POSIX's `readlinkat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    unsafe { readlinkat(start.as_raw_fd(), path).map(Into::into) }
}
