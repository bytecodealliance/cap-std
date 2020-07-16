use super::compute_oflags;
use crate::fs::{is_dir_options, stat_unchecked, FollowSymlinks, OpenOptions, OpenUncheckedError};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::{fs, path::Path};
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

    let err = unsafe {
        match openat(start.as_raw_fd(), path, oflags, mode) {
            Ok(fd) => return Ok(fs::File::from_raw_fd(fd)),
            Err(err) => err,
        }
    };
    match err.raw_os_error() {
        Some(libc::ELOOP) | Some(libc::EMLINK) => Err(OpenUncheckedError::Symlink(err)),
        Some(libc::ENOENT) => Err(OpenUncheckedError::NotFound(err)),
        Some(libc::ENOTDIR) => {
            if is_dir_options(options)
                && stat_unchecked(start, path, FollowSymlinks::follow(!options.nofollow))
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false)
            {
                Err(OpenUncheckedError::Symlink(err))
            } else {
                Err(OpenUncheckedError::NotFound(err))
            }
        }
        _ => Err(OpenUncheckedError::Other(err)),
    }
}
