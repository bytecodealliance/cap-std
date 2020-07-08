use super::compute_oflags;
use crate::fs::OpenOptions;
use crate::std::fs::OpenUncheckedError;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::{ffi::OsStr, fs, path::Path};
use yanix::file::{openat, Mode};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let oflags = compute_oflags(options).map_err(OpenUncheckedError::Other)?;

    #[allow(clippy::useless_conversion)]
    let mode = Mode::from_bits_truncate(options.ext.mode as _);

    // POSIX's `openat` with an empty path returns `ENOENT`, so use "." instead.
    let path = if path.components().next().is_none() {
        OsStr::new(".")
    } else {
        path.as_ref()
    };

    let err = unsafe {
        match openat(start.as_raw_fd(), path, oflags, mode) {
            Ok(fd) => return Ok(fs::File::from_raw_fd(fd)),
            Err(err) => err,
        }
    };
    match err.raw_os_error() {
        Some(libc::ELOOP) | Some(libc::EMLINK) => Err(OpenUncheckedError::Symlink(err)),
        _ => Err(OpenUncheckedError::Other(err)),
    }
}
