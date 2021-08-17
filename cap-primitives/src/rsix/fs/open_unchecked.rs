use super::compute_oflags;
use crate::fs::{stat_unchecked, OpenOptions, OpenUncheckedError};
use crate::AmbientAuthority;
use io_lifetimes::{AsFilelike, FromFd};
use rsix::fs::{cwd, openat, Mode};
use rsix::io;
use std::fs;
use std::path::Path;
#[cfg(any(target_os = "android", target_os = "linux"))]
use {crate::fs::ensure_cloexec, io_lifetimes::AsFd};

/// *Unsandboxed* function similar to `open`, but which does not perform
/// sandboxing.
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

            return Ok(fs::File::from_fd(file.into()));
        }
        Err(err) => err,
    };
    match err {
        // `ELOOP` is the POSIX standard and most widely used error code to
        // indicate that a symlink was found when `O_NOFOLLOW` was set.
        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd")))]
        io::Error::LOOP => Err(OpenUncheckedError::Symlink(err.into(), ())),

        // FreeBSD and similar (but not Darwin) use `EMLINK`.
        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        io::Error::MLINK => Err(OpenUncheckedError::Symlink(err.into(), ())),

        // NetBSD uses `EFTYPE`.
        #[cfg(any(target_os = "netbsd"))]
        io::Error::FTYPE => Err(OpenUncheckedError::Symlink(err.into(), ())),

        io::Error::NOENT => Err(OpenUncheckedError::NotFound(err.into())),
        io::Error::NOTDIR => {
            if options.dir_required
                && stat_unchecked(start, path, options.follow)
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false)
            {
                Err(OpenUncheckedError::Symlink(err.into(), ()))
            } else {
                Err(OpenUncheckedError::NotFound(err.into()))
            }
        }
        _ => Err(OpenUncheckedError::Other(err.into())),
    }
}

/// *Unsandboxed* function similar to `open`, but which does not perform
/// sandboxing.
#[inline]
pub(crate) fn open_ambient_impl(
    path: &Path,
    options: &OpenOptions,
    _ambient_authority: AmbientAuthority,
) -> Result<fs::File, OpenUncheckedError> {
    open_unchecked(&cwd().as_filelike_view::<fs::File>(), path, options)
}
