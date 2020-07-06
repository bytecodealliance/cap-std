use super::compute_oflags;
use crate::fs::OpenOptions;
use std::{
    ffi::OsStr,
    path::Path,
    fs, io,
    os::unix::io::{AsRawFd, FromRawFd},
};
use yanix::file::{openat, Mode};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    let oflags = compute_oflags(options);

    #[allow(clippy::useless_conversion)]
    let mode = Mode::from_bits_truncate(options.ext.mode as _);

    // POSIX's `openat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.components().next().is_none() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    unsafe {
        let fd = openat(start.as_raw_fd(), path, oflags, mode)?;
        Ok(fs::File::from_raw_fd(fd))
    }
}
