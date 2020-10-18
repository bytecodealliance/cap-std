//! Linux has an `O_PATH` flag which allows opening a file without necessary
//! having read or write access to it; we can use that with `openat2` and
//! `fstat` to perform a fast sandboxed `stat`.

use super::file_metadata;
use crate::fs::{manually, open_beneath, FollowSymlinks, Metadata, OpenOptions};
use std::{fs, io, path::Path};

/// Use `openat2` with `O_PATH` and `fstat`. If that's not available, fallback
/// to `manually::stat`.
pub(crate) fn stat_impl(
    start: &fs::File,
    path: &Path,
    follow: FollowSymlinks,
) -> io::Result<Metadata> {
    use std::os::unix::fs::OpenOptionsExt;

    // Open the path with `O_PATH`. Use `read(true)` even though we don't need
    // `read` permissions, because Rust's libstd requires an access mode, and
    // Linux ignores `O_RDONLY` with `O_PATH`.
    let result = open_beneath(
        start,
        path,
        OpenOptions::new()
            .read(true)
            .follow(follow)
            .custom_flags(libc::O_PATH),
    );

    // If that worked, call `fstat`.
    match result {
        Ok(file) => file_metadata(&file),
        Err(err) => match err.raw_os_error() {
            // `ENOSYS` from `open_beneath` means `openat2` is unavailable
            // and we should use a fallback.
            Some(libc::ENOSYS) => manually::stat(start, path, follow),
            _ => Err(err),
        },
    }
}
