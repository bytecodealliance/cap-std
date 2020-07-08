use std::{
    ffi::OsStr,
    fs, io,
    path::Path,
    os::unix::io::AsRawFd,
};
use yanix::file::symlinkat;

/// *Unsandboxed* function similar to `symlink`, but which does not perform sandboxing.
pub(crate) fn symlink_unchecked(
    old_path: &Path,
    new_start: &fs::File,
    new_path: &Path,
) -> io::Result<()> {
    // POSIX's `symlinkat` with an empty path returns `ENOENT`, so use "." instead.
    let new_path = if new_path.as_os_str().is_empty() {
        OsStr::new(".")
    } else {
        new_path.as_ref()
    };

    // TODO: Remove the .as_ref() when a newer yanix obviates it.
    unsafe { symlinkat(old_path, new_start.as_raw_fd(), new_path.as_ref()) }
}
