use std::{
    ffi::OsStr,
    fs, io,
    path::Path,
    os::unix::io::AsRawFd,
};
use yanix::file::{mkdirat, Mode};

/// *Unsandboxed* function similar to `mkdir`, but which does not perform sandboxing.
pub(crate) fn mkdir_unchecked(
    start: &fs::File,
    path: &Path,
) -> io::Result<()> {
    // POSIX's `mkdirat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.components().next().is_none() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    unsafe { mkdirat(start.as_raw_fd(), path, Mode::from_bits(0o777).unwrap()) }
}
