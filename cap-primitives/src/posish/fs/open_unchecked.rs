use super::compute_oflags;
use crate::fs::{stat_unchecked, OpenOptions, OpenUncheckedError};
use io_lifetimes::FromFd;
use posish::fs::{openat, Mode};
use posish::io::Errno;
use std::{fs, path::Path};
#[cfg(any(target_os = "android", target_os = "linux"))]
use {crate::fs::ensure_cloexec, io_lifetimes::AsFd};

/// *Unsandboxed* function similar to `open`, but which does not perform sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let oflags = compute_oflags(options).map_err(OpenUncheckedError::Other)?;

    #[allow(clippy::useless_conversion)]
    let mode = Mode::from_bits_truncate(options.ext.mode as _);

    let err = match openat(start, path, oflags, mode) {
        Ok(file) => {
            #[cfg(any(target_os = "android", target_os = "linux"))]
            ensure_cloexec(file.as_fd()).map_err(OpenUncheckedError::Other)?;

            return Ok(fs::File::from_fd(file));
        }
        Err(err) => err,
    };
    match Errno::from_io_error(&err) {
        // `ELOOP` is the POSIX standard and most widely used error code to
        // indicate that a symlink was found when `O_NOFOLLOW` was set.
        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd")))]
        Some(Errno::LOOP) => Err(OpenUncheckedError::Symlink(err, ())),

        // FreeBSD and similar (but not Darwin) use `EMLINK`.
        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        Some(Errno::MLINK) => Err(OpenUncheckedError::Symlink(err, ())),

        // NetBSD uses `EFTYPE`.
        #[cfg(any(target_os = "netbsd"))]
        Some(Errno::FTYPE) => Err(OpenUncheckedError::Symlink(err, ())),

        Some(Errno::NOENT) => Err(OpenUncheckedError::NotFound(err)),
        Some(Errno::NOTDIR) => {
            if options.dir_required
                && stat_unchecked(start, path, options.follow)
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false)
            {
                Err(OpenUncheckedError::Symlink(err, ()))
            } else {
                Err(OpenUncheckedError::NotFound(err))
            }
        }
        _ => Err(OpenUncheckedError::Other(err)),
    }
}
