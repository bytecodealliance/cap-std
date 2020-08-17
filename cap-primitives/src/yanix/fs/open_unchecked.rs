use super::compute_oflags;
#[cfg(target_os = "linux")]
use crate::fs::ensure_cloexec;
use crate::fs::{stat_unchecked, OpenOptions, OpenUncheckedError};
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
            Ok(fd) => {
                #[cfg(target_os = "linux")]
                ensure_cloexec(fd).map_err(OpenUncheckedError::Other)?;

                return Ok(fs::File::from_raw_fd(fd));
            }
            Err(err) => err,
        }
    };
    match err.raw_os_error() {
        // `ELOOP` is the POSIX standard and most widely used error code to
        // indicate that a symlink was found when `O_NOFOLLOW` was set.
        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd")))]
        Some(libc::ELOOP) => Err(OpenUncheckedError::Symlink(err)),

        // FreeBSD and similar (but not Darwin) use `EMLINK`.
        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        Some(libc::EMLINK) => Err(OpenUncheckedError::Symlink(err)),

        // NetBSD uses `EFTYPE`.
        #[cfg(any(target_os = "netbsd"))]
        Some(libc::EFTYPE) => Err(OpenUncheckedError::Symlink(err)),

        Some(libc::ENOENT) => Err(OpenUncheckedError::NotFound(err)),
        Some(libc::ENOTDIR) => {
            if options.dir_required
                && stat_unchecked(start, path, options.follow)
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
