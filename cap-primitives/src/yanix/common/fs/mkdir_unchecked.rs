use std::{ffi::OsStr, fs, io, os::unix::io::AsRawFd, path::Path};
use yanix::file::{mkdirat, Mode};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(start: &fs::File, path: &Path) -> io::Result<()> {
    // POSIX's `mkdirat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    unsafe { mkdirat(start.as_raw_fd(), path, Mode::from_bits(0o777).unwrap()) }
}
