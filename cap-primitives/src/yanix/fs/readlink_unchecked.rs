use std::{
    fs, io,
    os::unix::io::AsRawFd,
    path::{Path, PathBuf},
};
use yanix::file::readlinkat;

/// *Unsandboxed* function similar to `readlink`, but which does not perform sandboxing.
pub(crate) fn readlink_unchecked(start: &fs::File, path: &Path) -> io::Result<PathBuf> {
    unsafe { readlinkat(start.as_raw_fd(), path).map(Into::into) }
}
