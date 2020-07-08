#[cfg(unix)]
use std::os::unix::io::AsRawFd;
use std::{ffi::OsStr, fs, io, path::PathBuf};
use yanix::file::readlinkat;

const MAX_SYMLINK_EXPANSIONS: u8 = 40;

/// This is a wrapper around `readlinkat` which helps enforce the symlink
/// expansion limit, and which handles the low-level details.
pub(crate) fn resolve_symlink_at(
    base: &fs::File,
    name: &OsStr,
    symlink_count: &mut u8,
) -> io::Result<PathBuf> {
    if *symlink_count == MAX_SYMLINK_EXPANSIONS {
        return too_many_symlinks();
    }

    let destination = unsafe { readlinkat(base.as_raw_fd(), name) }?;

    *symlink_count += 1;

    Ok(PathBuf::from(destination))
}

#[cold]
fn too_many_symlinks() -> io::Result<PathBuf> {
    Err(io::Error::from_raw_os_error(libc::ELOOP))
}
